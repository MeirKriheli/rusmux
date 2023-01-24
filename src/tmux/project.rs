use crate::error::AppError;
use crate::project_config::ProjectConfig;
use crate::project_config::Window;
use super::commands::Commands;
use std::fmt;
use std::process::Command;

pub const TMUX_BIN: &str = "tmux";
const READ_ERROR: &str = "Cannot get tmux config options";

#[derive(Debug)]
struct Tmux {
    base_index: usize,
    pane_base_index: usize,
}

impl Tmux {
    /// Create a new `Tmux` instance with the proposed `base-index` and `pane-base-index`.
    fn new(base_index: usize, pane_base_index: usize) -> Self {
        Self {
            base_index,
            pane_base_index,
        }
    }

    /// Create a new `Tmux` instance getting the values of `base-index` and `pane-base-index` from
    /// the installed tmux config.
    fn new_from_config() -> Result<Self, AppError> {
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
            ])
            .output()
            .map_err(|_| AppError::Message(READ_ERROR.into()))?
            .stdout;

        let values: Vec<usize> = String::from_utf8(output)
            .map_err(|_| AppError::Message(READ_ERROR.into()))?
            .lines()
            .map(|line| line.split(' ').nth(1).unwrap().parse::<usize>().unwrap())
            .collect();

        if values.len() != 2 {
            return Err(AppError::Message(READ_ERROR.into()));
        }

        Ok(Self::new(values[0], values[1]))
    }
}


#[derive(Debug)]
pub struct TmuxProject<'a> {
    tmux: Tmux,
    project: &'a ProjectConfig,
}

impl<'a> TmuxProject<'a> {
    pub fn new(project: &'a ProjectConfig) -> Result<Self, AppError> {
        let tmux = Tmux::new_from_config()?;
        Ok(TmuxProject { tmux, project })
    }

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

        commands.push(self.get_attach_session_command());

        commands.push(Commands::ProjectEvent {
            event_name: "exit",
            on_event: &self.project.on_project_exit,
        });

        commands
    }

    fn get_attach_session_command(&self) -> Commands {
        Commands::AttachSession {
            session_name: &self.project.project_name,
        }
    }

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

    fn session_exists(&self) -> Result<bool, AppError> {
        let res = Command::new(TMUX_BIN)
            .env_remove("TMUX")
            .arg("has-session")
            .arg("-t")
            .arg(&self.project.project_name)
            .output()?;

        Ok(res.status.success())
    }

    pub fn run(&self) -> Result<(), AppError> {
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
}

impl<'a> fmt::Display for TmuxProject<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let joined = self
            .get_commands()
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", joined)
    }
}
