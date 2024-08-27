//! The various tmux operations commands.
use super::project::TMUX_BIN;
use super::TmuxError;

use clap::crate_name;
use shlex::Shlex;
use std::env::set_current_dir;
use std::process::Command;
use std::{env, fmt};

/// The commands. Implemented as an enum instead of traits/structs
/// to prevent dynamic dispatch.
///
/// Implements [`Display`](std::fmt::Display), formatting the commands a shell
/// command which is used to display the shell script for the `debug`
/// cli command.
#[derive(Debug)]
pub(crate) enum Commands<'a> {
    /// Start the server and change into the work directory if specified.
    Server {
        project_name: &'a str,
        project_root: &'a Option<String>,
    },
    /// Runs the various `on_project_<event>` commands.
    ProjectEvent {
        event_name: &'a str,
        on_event: &'a Option<Vec<String>>,
    },
    /// Start the new tmux session, and cd again (for tmux < 1.9 compat).
    Session {
        project_name: &'a str,
        first_window_name: Option<&'a str>,
    },
    /// `send-keys` command.
    SendKeys {
        command: String,
        session_name: &'a str,
        window_index: usize,
        pane_index: Option<usize>,
        comment: Option<String>,
    },
    /// `new-window` command.
    NewWindow {
        session_name: &'a str,
        window_name: &'a str,
        window_index: usize,
        project_root: &'a Option<String>,
    },
    /// `split-window` command.
    SplitWindow {
        session_name: &'a str,
        window_index: usize,
        project_root: &'a Option<String>,
    },
    /// `select-layout` command.
    SelectLayout {
        session_name: &'a str,
        window_index: usize,
        layout: &'a str,
    },
    /// `select-window` command.
    SelectWindow {
        session_name: &'a str,
        window_index: usize,
    },
    /// `select-pane` command
    SelectPane {
        session_name: &'a str,
        window_index: usize,
        pane_index: usize,
    },
    /// Attaches to a session using `attach-sesssion` or `switch-client`,
    /// depends upon already running inside a tmux session or out of it.
    AttachSession { session_name: &'a str },
    /// `kill-session` command
    StopSession { session_name: &'a str },
    /// Set a hook for tmux events
    SetHook {
        session_name: &'a str,
        hook_name: &'a str,
        hook_command: String,
    },
}

impl<'a> Commands<'a> {
    fn fmt_server_command(
        f: &mut fmt::Formatter<'_>,
        project_name: &'a str,
        project_root: &'a Option<String>,
    ) -> fmt::Result {
        let shebang = env::var("SHELL").map(|x| format!("#!{x}")).ok();
        let cd_command = match project_root {
            Some(project_root) => format!("\ncd {project_root}"),
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
        write!(f, "\n# Run on_project_{event_name} command(s)\n{commands}")
    }

    fn fmt_session_command(
        f: &mut fmt::Formatter<'_>,
        project_name: &'a str,
        first_window_name: Option<&'a str>,
    ) -> fmt::Result {
        let window_param = first_window_name.map_or("".into(), |n| format!(" -n {n}"));
        write!(
            f,
            "\n# Create new session and first window\n\
            TMUX= {TMUX_BIN} new-session -d -s {project_name}{window_param}"
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
        let formatted_pane_index = pane_index.map_or("".into(), |idx| format!(".{idx}"));
        let comment = comment.map_or("".into(), |c| format!("\n# {c}\n"));
        let escaped = shell_escape::escape(command.into());
        write!(
            f,
            "{comment}{TMUX_BIN} send-keys -t {session_name}:{window_index}{formatted_pane_index} {escaped} C-m"
        )
    }

    fn get_cd_root_flag(project_root: &Option<String>) -> String {
        match project_root {
            Some(dir) => format!(" -c {dir}"),
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
            "\n# Create \"{window_name}\" window \n{TMUX_BIN} new-window{cd_root} -t {session_name}:{window_index} -n {window_name}"
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
            "{TMUX_BIN} splitw{cd_root} -t {session_name}:{window_index}"
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
            "{TMUX_BIN} select-layout -t {session_name}:{window_index} {layout}"
        )
    }

    fn fmt_select_window(
        f: &mut fmt::Formatter,
        session_name: &str,
        window_index: usize,
    ) -> Result<(), fmt::Error> {
        write!(
            f,
            "{TMUX_BIN} select-window -t {session_name}:{window_index}"
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
            "{TMUX_BIN} select-pane -t {session_name}:{window_index}.{pane_index}"
        )
    }

    fn fmt_attach_session(f: &mut fmt::Formatter, session_name: &str) -> Result<(), fmt::Error> {
        write!(
            f,
            "\nif [ -z \"$TMUX\" ]; then\n  {TMUX_BIN} -u attach-session -t {session_name}\n\
            else\n  {TMUX_BIN} -u switch-client -t {session_name}\nfi"
        )
    }

    fn fmt_stop_session(f: &mut fmt::Formatter, session_name: &str) -> Result<(), fmt::Error> {
        write!(f, "{TMUX_BIN} kill-session -t {session_name}")
    }

    fn fmt_set_hook(
        f: &mut fmt::Formatter,
        session_name: &str,
        hook_name: &str,
        hook_command: &str,
    ) -> Result<(), fmt::Error> {
        write!(
            f,
            "{TMUX_BIN} set-hook -t {session_name} {hook_name} \"{hook_command}\""
        )
    }

    /// Runs the command, based on the enum values.
    pub fn run(&self) -> Result<(), TmuxError> {
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
            } => Commands::run_split_window(session_name, *window_index, project_root),
            Commands::SelectLayout {
                session_name,
                window_index,
                layout,
            } => Commands::run_select_layout(session_name, *window_index, layout),
            Commands::SelectWindow {
                session_name,
                window_index,
            } => Commands::run_select_window(session_name, *window_index),
            Commands::SelectPane {
                session_name,
                window_index,
                pane_index,
            } => Commands::run_select_pane(session_name, *window_index, *pane_index),
            Commands::AttachSession { session_name } => Commands::run_attach_session(session_name),
            Commands::StopSession { session_name } => Commands::run_stop_session(session_name),
            Commands::SetHook {
                session_name,
                hook_name,
                hook_command,
            } => Commands::run_set_hook(session_name, hook_name, hook_command),
        }
    }

    fn run_server_command(project_root: &'a Option<String>) -> Result<(), TmuxError> {
        Command::new(TMUX_BIN).arg("start-server").status()?;
        if let Some(root_dir) = project_root {
            set_current_dir(shellexpand::full(root_dir)?.as_ref())?;
        }
        Ok(())
    }

    fn run_project_event(on_event: &Option<Vec<String>>) -> Result<(), TmuxError> {
        if let Some(commands) = on_event {
            for command in commands {
                let mut parts = Shlex::new(command);
                let cmd_opt = parts.next();
                if let Some(cmd) = cmd_opt {
                    let res = Command::new(cmd).args(parts.collect::<Vec<_>>()).status();
                    if res.is_err() {
                        eprintln!("Error executing command {command}");
                    }
                }
            }
        }
        Ok(())
    }

    fn run_session_command(
        project_name: &str,
        first_window_name: &Option<&str>,
    ) -> Result<(), TmuxError> {
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
            Err(TmuxError::Message("Cannot start session".to_string()))
        }
    }

    fn run_send_keys(
        command: &String,
        session_name: &str,
        window_index: usize,
        pane_index: &Option<usize>,
    ) -> Result<(), TmuxError> {
        let target_name = if let Some(pane_idx) = pane_index {
            format!("{session_name}:{window_index}.{pane_idx}")
        } else {
            format!("{session_name}:{window_index}")
        };
        let args = ["send-keys", "-t", &target_name, command, "C-m"];
        let res = Command::new(TMUX_BIN).args(args).status()?;

        if res.success() {
            Ok(())
        } else {
            Err(TmuxError::Message(format!(
                "Cannot run send-keys for {command}"
            )))
        }
    }

    fn run_new_window(
        session_name: &str,
        window_name: &str,
        window_index: usize,
        project_root: &Option<String>,
    ) -> Result<(), TmuxError> {
        let target_name = format!("{session_name}:{window_index}");
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
            Err(TmuxError::Message(format!(
                "Cannot create window {window_name}"
            )))
        }
    }

    fn run_split_window(
        session_name: &str,
        window_index: usize,
        project_root: &Option<String>,
    ) -> Result<(), TmuxError> {
        let target_name = format!("{session_name}:{window_index}");
        let mut args = vec!["splitw", "-t", &target_name];
        let expanded: String;
        if let Some(root_dir) = project_root {
            args.push("-c");
            expanded = shellexpand::full(root_dir)?.to_string();
            args.push(&expanded);
        }
        let res = Command::new(TMUX_BIN).args(args).status()?;

        if res.success() {
            Ok(())
        } else {
            Err(TmuxError::Message(format!(
                "Cannot split window {target_name}"
            )))
        }
    }

    fn run_select_layout(
        session_name: &str,
        window_index: usize,
        layout: &str,
    ) -> Result<(), TmuxError> {
        let target_name = format!("{session_name}:{window_index}");
        let args = vec!["select-layout", "-t", &target_name, layout];
        let res = Command::new(TMUX_BIN).args(args).status()?;

        if res.success() {
            Ok(())
        } else {
            Err(TmuxError::Message(format!(
                "Cannot select layout {layout} for window {target_name}"
            )))
        }
    }

    fn run_select_window(session_name: &str, window_index: usize) -> Result<(), TmuxError> {
        let target_name = format!("{session_name}:{window_index}");
        let args = vec!["select-window", "-t", &target_name];
        let res = Command::new(TMUX_BIN).args(args).status()?;

        if res.success() {
            Ok(())
        } else {
            Err(TmuxError::Message(format!(
                "Cannot select window {target_name}"
            )))
        }
    }

    fn run_select_pane(
        session_name: &str,
        window_index: usize,
        pane_index: usize,
    ) -> Result<(), TmuxError> {
        let target_name = format!("{session_name}:{window_index}.{pane_index}");
        let args = vec!["select-pane", "-t", &target_name];
        let res = Command::new(TMUX_BIN).args(args).status()?;

        if res.success() {
            Ok(())
        } else {
            Err(TmuxError::Message(format!(
                "Cannot select pane {target_name}"
            )))
        }
    }

    fn run_attach_session(session_name: &str) -> Result<(), TmuxError> {
        let param = if env::var("TMUX").is_ok() {
            "switch-client"
        } else {
            "attach-session"
        };
        let args = ["-u", param, "-t", session_name];
        let res = Command::new(TMUX_BIN).args(args).status()?;
        if res.success() {
            Ok(())
        } else {
            Err(TmuxError::Message(format!(
                "Cannot {param} to session {session_name}"
            )))
        }
    }

    fn run_stop_session(session_name: &str) -> Result<(), TmuxError> {
        let args = ["kill-session", "-t", session_name];
        let res = Command::new(TMUX_BIN).args(args).status()?;
        if res.success() {
            Ok(())
        } else {
            Err(TmuxError::Message(format!(
                "Cannot kill session {session_name}"
            )))
        }
    }

    fn run_set_hook(
        session_name: &str,
        hook_name: &str,
        hook_command: &str,
    ) -> Result<(), TmuxError> {
        let args = ["set-hook", "-t", session_name, hook_name, hook_command];
        let res = Command::new(TMUX_BIN).args(args).status()?;
        if res.success() {
            Ok(())
        } else {
            Err(TmuxError::Message(format!(
                "Cannot set {hook_name} hook for session {session_name}"
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
            Commands::StopSession { session_name } => Commands::fmt_stop_session(f, session_name),
            Commands::SetHook {
                session_name,
                hook_name,
                hook_command,
            } => Commands::fmt_set_hook(f, session_name, hook_name, hook_command),
        }
    }
}
