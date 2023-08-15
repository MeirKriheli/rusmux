//! Maps rusmux's [`ProjectConfig`] to tmux commands and operations.
use super::commands::Commands;
use super::TmuxError;
use super::TmuxVersion;
use crate::project_config::ProjectConfig;
use crate::project_config::Window;
use std::fmt;
use std::process::Command;

pub const TMUX_BIN: &str = "tmux";
const READ_ERROR: &str = "Cannot get tmux version and config options";

/// Stores Tmux configuration information.
///
/// Since user configuration can alter settings, e.g the base index of a
/// window (0, 1).
#[derive(Debug)]
struct Tmux {
    /// Base index for a new window.
    base_index: usize,
    /// Base index for a new pane.
    pane_base_index: usize,
    /// Tmux version
    version: TmuxVersion,
}

impl Tmux {
    /// Create a new `Tmux` instance with the proposed `base-index` and `pane-base-index`.
    fn new(base_index: usize, pane_base_index: usize, version: TmuxVersion) -> Self {
        Self {
            base_index,
            pane_base_index,
            version,
        }
    }

    /// Create a new `Tmux` instance getting the values of `base-index` and `pane-base-index`
    /// from the installed tmux configuration.
    fn new_from_config() -> Result<Self, TmuxError> {
        let output = Command::new(TMUX_BIN)
            .args([
                "start",
                ";",
                "show",
                "-g",
                "base-index",
                ";",
                "show",
                "-g",
                "pane-base-index",
                ";",
                "display-message",
                "-p",
                "#{version}",
            ])
            .output()
            .map_err(|_| TmuxError::Message(READ_ERROR.into()))?
            .stdout;

        let binding =
            String::from_utf8(output).map_err(|_| TmuxError::Message(READ_ERROR.into()))?;
        let lines: Vec<&str> = binding.lines().collect();

        let values: Vec<usize> = lines
            .iter()
            .take(2)
            .map(|line| line.split(' ').nth(1).unwrap().parse::<usize>().unwrap())
            .collect();

        match lines.len() {
            2 => Ok(Self::new(values[0], values[1], None.into())),
            3 => Ok(Self::new(values[0], values[1], Some(lines[2]).into())),
            _ => Err(TmuxError::Message(READ_ERROR.into())),
        }
    }
}

/// The tmux project, generates the required commands based on
/// [ProjectConfig] and [Tmux] settings.
#[derive(Debug)]
pub struct TmuxProject<'a> {
    tmux: Tmux,
    project: &'a ProjectConfig,
}

impl<'a> TmuxProject<'a> {
    /// Creates a new Tmux project from a [`ProjectConfig`].
    pub fn new(project: &'a ProjectConfig) -> Result<Self, TmuxError> {
        let tmux = Tmux::new_from_config()?;
        Ok(TmuxProject { tmux, project })
    }

    /// Gets the list of commands for the project, and runs them:
    ///
    /// - Commands to create the session if not running already.
    /// - Commands to attach to an existing session.
    pub fn run(&self) -> Result<(), TmuxError> {
        let cmds = if self.session_exists()? {
            vec![self.get_attach_session_command()]
        } else {
            self.get_commands()
        };
        for cmd in cmds {
            cmd.run()?;
        }
        Ok(())
    }

    /// Stops the project's session, and run `on_project_stop`
    /// (if specified).
    pub fn stop(&self) -> Result<(), TmuxError> {
        let cmds = self.get_stop_session_commands();
        for cmd in cmds {
            cmd.run()?;
        }
        Ok(())
    }

    /// Helper returning the [`Commands`] for creating the project's session.
    fn get_commands(&'a self) -> Vec<Commands> {
        let project_name = &self.project.project_name;

        let first_window_name = self
            .project
            .windows
            .as_ref()
            .and_then(|windows| windows.first())
            .map(|w| w.name.as_str());

        let mut commands = vec![
            Commands::Server {
                project_name,
                project_root: &self.project.project_root,
            },
            Commands::ProjectEvent {
                event_name: "start",
                on_event: &self.project.on_project_start,
            },
            Commands::Session {
                project_name,
                first_window_name,
            },
        ];

        if let Some(project_root) = &self.project.project_root {
            commands.push(Commands::SendKeys {
                command: format!("cd {}", &project_root),
                session_name: project_name,
                window_index: self.tmux.base_index,
                pane_index: None,
                comment: Some(
                    "Manually switch to root directory if required to support tmux < 1.9".into(),
                ),
            });
        }

        if let Some(windows) = self.project.windows.as_ref() {
            windows.iter().enumerate().for_each(|(idx, w)| {
                commands.extend(self.get_window_commands(idx, w));
            });

            // select 1st window and 1st pane
            commands.push(Commands::SelectWindow {
                session_name: project_name,
                window_index: self.tmux.base_index,
            });

            commands.push(Commands::SelectPane {
                session_name: project_name,
                window_index: self.tmux.base_index,
                pane_index: self.tmux.pane_base_index,
            })
        }

        if self.tmux.version >= TmuxVersion::Version(2, 6) {
            let hook_cmd = self.get_layout_hooks_command();

            if let Some(cmd) = hook_cmd {
                commands.push(cmd);
            }
        }

        commands.push(self.get_attach_session_command());

        commands.push(Commands::ProjectEvent {
            event_name: "exit",
            on_event: &self.project.on_project_exit,
        });

        commands
    }

    /// Helper returning the [`Commands`] for attaching to an
    /// already running session.
    fn get_attach_session_command(&self) -> Commands {
        Commands::AttachSession {
            session_name: &self.project.project_name,
        }
    }

    /// Helper returning the [`Commands`] for stopping to an
    /// already running session.
    fn get_stop_session_commands(&self) -> Vec<Commands> {
        vec![
            Commands::StopSession {
                session_name: &self.project.project_name,
            },
            Commands::ProjectEvent {
                event_name: "stop",
                on_event: &self.project.on_project_stop,
            },
        ]
    }

    /// Helper returning the commands for creating the [`Window`]s
    /// and the panes of the session. Called from
    /// [`get_commands`](`Self::get_commands`).
    ///
    /// The index of the window, `idx`, is `0` based, and is adjusted for
    /// the current tmux configuration for `base-index` and `pane-base-index`.
    fn get_window_commands(&'a self, idx: usize, w: &'a Window) -> Vec<Commands> {
        let mut commands = Vec::new();
        let project_name = &self.project.project_name;

        let window_idx = idx + self.tmux.base_index;
        if idx > 0 {
            commands.push(Commands::NewWindow {
                session_name: project_name,
                window_name: w.name.as_ref(),
                window_index: window_idx,
                project_root: &self.project.project_root,
            });
        }
        w.panes.iter().enumerate().for_each(|(pane_idx, pane)| {
            let pane_with_base_idx = pane_idx + self.tmux.pane_base_index;
            if pane_idx > 0 {
                commands.push(Commands::SplitWindow {
                    session_name: project_name,
                    window_index: window_idx,
                    project_root: &self.project.project_root,
                })
            }
            if let Some(pre_window) = &self.project.pre_window {
                pre_window.iter().enumerate().for_each(|(cmd_idx, cmd)| {
                    let comment = if idx == 0 && pane_idx == 0 && cmd_idx == 0 {
                        Some(format!("Continue \"{}\" window", w.name))
                    } else {
                        None
                    };
                    commands.push(Commands::SendKeys {
                        command: cmd.clone(),
                        session_name: project_name,
                        window_index: window_idx,
                        pane_index: Some(pane_with_base_idx),
                        comment,
                    });
                })
            }
            if let Some(pane_cmd) = pane {
                commands.push(Commands::SendKeys {
                    command: pane_cmd.clone(),
                    session_name: project_name,
                    window_index: window_idx,
                    pane_index: Some(pane_with_base_idx),
                    comment: None,
                });
            }
        });

        if w.panes.len() > 1 {
            commands.push(Commands::SelectLayout {
                session_name: project_name,
                window_index: window_idx,
                layout: &w.layout,
            });

            commands.push(Commands::SelectPane {
                session_name: project_name,
                window_index: window_idx,
                pane_index: self.tmux.pane_base_index,
            })
        }
        commands
    }

    /// Helper checking if the project's session is already running by
    /// utilizing `tmux has-session`.
    fn session_exists(&self) -> Result<bool, TmuxError> {
        let res = Command::new(TMUX_BIN)
            .env_remove("TMUX")
            .arg("has-session")
            .arg("-t")
            .arg(&self.project.project_name)
            .output()?;

        Ok(res.status.success())
    }

    /// Get to hook commands to correctly apply layouts
    fn get_layout_hooks_command(&self) -> Option<Commands> {
        // No windows? No need to set layouts
        if self.project.windows.as_ref().is_none() {
            return None;
        }

        let windows = self.project.windows.as_ref().unwrap();
        if windows.len() == 0 {
            return None;
        }

        let mut hook_commands: Vec<String> = vec![
            "set main-pane-height 66%".into(),
            "set main-pane-width 66%".into(),
        ];
        windows.iter().enumerate().for_each(|(idx, w)| {
            // Need to select window before applying layout
            hook_commands.push(format!("selectw -t {}", self.tmux.base_index + idx));
            hook_commands.push(format!("selectl {}", w.layout));
            if idx > 0 {
                hook_commands.push("selectw -l".into());
            }
        });

        // Once done, unset the hook
        hook_commands.push(format!(
            "set-hook -u -t {} client-session-changed",
            self.project.project_name
        ));

        let hook_command = hook_commands.join(";");

        Some(Commands::SetHook {
            session_name: &self.project.project_name,
            hook_name: "client-session-changed",
            hook_command,
        })
    }
}

impl<'a> fmt::Display for TmuxProject<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let joined = self
            .get_commands()
            .iter()
            .map(|x| format!("{x}"))
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{joined}")
    }
}
