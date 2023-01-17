use crate::error::AppError;
use crate::project::Project;
use crate::window::Window;
use clap::crate_name;
use shlex::Shlex;
use std::env;
use std::env::set_current_dir;
use std::fmt;
use std::process::Command;

const TMUX_BIN: &str = "tmux";
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
enum Commands<'a> {
    /// Start the server and cd to the work directory if available
    Server {
        project_name: &'a str,
        project_root: &'a Option<String>,
    },
    /// Run on_project_<event> commands
    ProjectEvent {
        event_name: &'a str,
        on_event: &'a Option<Vec<String>>,
    },
    /// Start the new tmux session, and cd again (for tmux < 1.9 compat)
    Session {
        project_name: &'a str,
        first_window_name: Option<&'a str>,
    },
    /// Send Keys command
    SendKeys {
        command: String,
        session_name: &'a str,
        window_index: usize,
        pane_index: Option<usize>,
        comment: Option<String>,
    },
    NewWindow {
        session_name: &'a str,
        window_name: &'a str,
        window_index: usize,
        project_root: &'a Option<String>,
    },
    SplitWindow {
        session_name: &'a str,
        window_index: usize,
        project_root: &'a Option<String>,
    },
    SelectLayout {
        session_name: &'a str,
        window_index: usize,
        layout: &'a str,
    },
    SelectWindow {
        session_name: &'a str,
        window_index: usize,
    },
    SelectPane {
        session_name: &'a str,
        window_index: usize,
        pane_index: usize,
    },
    AttachSession {
        session_name: &'a str,
    },
}

impl<'a> Commands<'a> {
    fn fmt_server_command(
        f: &mut fmt::Formatter<'_>,
        project_name: &'a str,
        project_root: &'a Option<String>,
    ) -> fmt::Result {
        let shebang = env::var("SHELL").map(|x| format!("#!{}", x)).ok();
        let cd_command = match project_root {
            Some(project_root) => format!("\ncd {}", project_root),
            None => "".into(),
        };

        write!(
            f,
            "{}\n\
             #\n\
             # {} {} project\n\n\
             {} start-server\
             {}",
            shebang.unwrap_or_else(|| "".into()),
            crate_name!(),
            project_name,
            TMUX_BIN,
            cd_command
        )
    }

    fn fmt_project_command(
        f: &mut fmt::Formatter<'_>,
        event_name: &'a str,
        on_event: &'a Option<Vec<String>>,
    ) -> fmt::Result {
        let commands = on_event.as_ref().map_or(String::from(""), |v| v.join("\n"));
        write!(
            f,
            "\n# Run on_project_{} command(s)\n{}",
            event_name, commands
        )
    }

    fn fmt_session_command(
        f: &mut fmt::Formatter<'_>,
        project_name: &'a str,
        first_window_name: Option<&'a str>,
    ) -> fmt::Result {
        let window_param = first_window_name.map_or("".into(), |n| format!(" -n {}", n));
        write!(
            f,
            "\n# Create new session and first window\n\
            TMUX= {} new-session -d -s {}{}",
            TMUX_BIN, project_name, window_param
        )
    }

    fn fmt_send_keys(
        f: &mut fmt::Formatter,
        command: &str,
        session_name: &str,
        window_index: usize,
        pane_index: Option<usize>,
        comment: Option<&str>,
    ) -> fmt::Result {
        let formatted_pane_index = pane_index.map_or("".into(), |idx| format!(".{}", idx));
        let comment = comment.map_or("".into(), |c| format!("\n# {}\n", c));
        let escaped = shell_escape::escape(command.into());
        write!(
            f,
            "{}{} send-keys -t {}:{}{} {} C-m",
            comment, TMUX_BIN, session_name, window_index, formatted_pane_index, escaped
        )
    }

    fn get_cd_root_flag(project_root: &Option<String>) -> String {
        match project_root {
            Some(dir) => format!(" -c {}", dir),
            None => "".into(),
        }
    }

    fn fmt_new_window(
        f: &mut fmt::Formatter,
        session_name: &str,
        window_name: &str,
        window_index: usize,
        project_root: &Option<String>,
    ) -> fmt::Result {
        let cd_root = Commands::get_cd_root_flag(project_root);
        write!(
            f,
            "\n# Create \"{}\" window \n{} new-window{} -t {}:{} -n {}",
            window_name, TMUX_BIN, cd_root, session_name, window_index, window_name
        )
    }

    fn fmt_split_window(
        f: &mut fmt::Formatter,
        session_name: &str,
        window_index: usize,
        project_root: &Option<String>,
    ) -> Result<(), fmt::Error> {
        let cd_root = Commands::get_cd_root_flag(project_root);
        write!(
            f,
            "{} splitw{} -t {}:{}",
            TMUX_BIN, cd_root, session_name, window_index
        )
    }

    fn fmt_select_layout(
        f: &mut fmt::Formatter,
        session_name: &str,
        window_index: usize,
        layout: &str,
    ) -> Result<(), fmt::Error> {
        write!(
            f,
            "{} select-layout -t {}:{} {}",
            TMUX_BIN, session_name, window_index, layout
        )
    }

    fn fmt_select_window(
        f: &mut fmt::Formatter,
        session_name: &str,
        window_index: usize,
    ) -> Result<(), fmt::Error> {
        write!(
            f,
            "{} select-window -t {}:{}",
            TMUX_BIN, session_name, window_index
        )
    }

    fn fmt_select_pane(
        f: &mut fmt::Formatter,
        session_name: &str,
        window_index: usize,
        pane_index: usize,
    ) -> Result<(), fmt::Error> {
        write!(
            f,
            "{} select-pane -t {}:{}.{}",
            TMUX_BIN, session_name, window_index, pane_index
        )
    }

    fn fmt_attach_session(f: &mut fmt::Formatter, session_name: &str) -> Result<(), fmt::Error> {
        write!(
            f,
            "\nif [ -z \"$TMUX\" ]; then\n  {} -u attach-session -t {}\n\
            else\n  {} -u switch-client -t {}\nfi",
            TMUX_BIN, session_name, TMUX_BIN, session_name
        )
    }

    fn run(&self) -> Result<(), AppError> {
        match self {
            Commands::Server {
                project_name: _,
                project_root,
            } => Commands::run_server_command(project_root),
            Commands::ProjectEvent {
                event_name: _,
                on_event,
            } => Commands::run_project_event(on_event),
            Commands::Session {
                project_name,
                first_window_name,
            } => Commands::run_session_command(project_name, first_window_name),
            Commands::SendKeys {
                command,
                session_name,
                window_index,
                pane_index,
                comment: _,
            } => Commands::run_send_keys(command, session_name, *window_index, pane_index),
            Commands::NewWindow {
                session_name,
                window_name,
                window_index,
                project_root,
            } => Commands::run_new_window(session_name, window_name, *window_index, project_root),
            Commands::SplitWindow {
                session_name,
                window_index,
                project_root,
            } => todo!(),
            Commands::SelectLayout {
                session_name,
                window_index,
                layout,
            } => todo!(),
            Commands::SelectWindow {
                session_name,
                window_index,
            } => todo!(),
            Commands::SelectPane {
                session_name,
                window_index,
                pane_index,
            } => todo!(),
            Commands::AttachSession { session_name } => todo!(),
        }
    }

    fn run_server_command(project_root: &'a Option<String>) -> Result<(), AppError> {
        Command::new(TMUX_BIN).arg("start-server").status()?;
        if let Some(root_dir) = project_root {
            set_current_dir(shellexpand::full(root_dir)?.as_ref())?;
        }
        Ok(())
    }

    fn run_project_event(on_event: &Option<Vec<String>>) -> Result<(), AppError> {
        if let Some(commands) = on_event {
            for command in commands {
                let mut parts = Shlex::new(command);
                let cmd_opt = parts.next();
                if let Some(cmd) = cmd_opt {
                    let res = Command::new(cmd).args(parts.collect::<Vec<_>>()).status();
                    if res.is_err() {
                        eprintln!("Error executing command {}", command);
                    }
                }
            }
        }
        Ok(())
    }

    fn run_session_command(
        project_name: &str,
        first_window_name: &Option<&str>,
    ) -> Result<(), AppError> {
        let mut session_args = vec!["new-session", "-d", "-s", project_name];
        if let Some(name) = first_window_name {
            session_args.push("-n");
            session_args.push(name);
        }
        let res = Command::new(TMUX_BIN)
            .env_remove("TMUX")
            .args(session_args)
            .status()?;

        if res.success() {
            Ok(())
        } else {
            Err(AppError::Message("Cannot start session".to_string()))
        }
    }

    fn run_send_keys(
        command: &String,
        session_name: &str,
        window_index: usize,
        pane_index: &Option<usize>,
    ) -> Result<(), AppError> {
        let target_name = if let Some(pane_idx) = pane_index {
            format!("{}:{}.{}", session_name, window_index, pane_idx)
        } else {
            format!("{}:{}", session_name, window_index)
        };
        let args = ["send-keys", "-t", &target_name, command, "C-m"];
        let res = Command::new(TMUX_BIN).args(args).status()?;

        if res.success() {
            Ok(())
        } else {
            Err(AppError::Message(format!(
                "Cannot run send-keys for {}",
                command
            )))
        }
    }

    fn run_new_window(
        session_name: &str,
        window_name: &str,
        window_index: usize,
        project_root: &Option<String>,
    ) -> Result<(), AppError> {
        let target_name = format!("{}:{}", session_name, window_index);
        let expanded: String;
        let mut args = vec!["new-window", "-t", &target_name, "-n", window_name];
        if let Some(root_dir) = project_root {
            args.push("-c");
            expanded = shellexpand::full(root_dir)?.to_string();
            args.push(&expanded);
        }
        let res = Command::new(TMUX_BIN).args(args).status()?;

        if res.success() {
            Ok(())
        } else {
            Err(AppError::Message(format!(
                "Cannot create window {}",
                window_name
            )))
        }
    }
}

impl<'a> fmt::Display for Commands<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Commands::Server {
                project_name,
                project_root,
            } => Commands::fmt_server_command(f, project_name, project_root),
            Commands::ProjectEvent {
                event_name,
                on_event,
            } => Commands::fmt_project_command(f, event_name, on_event),
            Commands::Session {
                project_name,
                first_window_name,
            } => Commands::fmt_session_command(f, project_name, *first_window_name),
            Commands::SendKeys {
                command,
                session_name,
                window_index,
                pane_index,
                comment,
            } => Commands::fmt_send_keys(
                f,
                command,
                session_name,
                *window_index,
                *pane_index,
                comment.as_ref().map(|x| &**x),
            ),
            Commands::NewWindow {
                session_name,
                window_name,
                window_index,
                project_root,
            } => {
                Commands::fmt_new_window(f, session_name, window_name, *window_index, project_root)
            }
            Commands::SplitWindow {
                session_name,
                window_index,
                project_root,
            } => Commands::fmt_split_window(f, session_name, *window_index, project_root),
            Commands::SelectLayout {
                session_name,
                window_index,
                layout,
            } => Commands::fmt_select_layout(f, session_name, *window_index, layout),
            Commands::SelectWindow {
                session_name,
                window_index,
            } => Commands::fmt_select_window(f, session_name, *window_index),
            Commands::SelectPane {
                session_name,
                window_index,
                pane_index,
            } => Commands::fmt_select_pane(f, session_name, *window_index, *pane_index),
            Commands::AttachSession { session_name } => {
                Commands::fmt_attach_session(f, session_name)
            }
        }
    }
}

#[derive(Debug)]
pub struct TmuxProject<'a> {
    tmux: Tmux,
    project: &'a Project,
}

impl<'a> TmuxProject<'a> {
    pub fn new(project: &'a Project) -> Result<Self, AppError> {
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

        commands.push(Commands::AttachSession {
            session_name: project_name,
        });

        commands.push(Commands::ProjectEvent {
            event_name: "exit",
            on_event: &self.project.on_project_exit,
        });

        commands
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
            if idx > 0 {
                commands.push(Commands::SplitWindow {
                    session_name: project_name,
                    window_index: pane_with_base_idx,
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

    pub fn run(&self) -> Result<(), AppError> {
        for cmd in self.get_commands() {
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
