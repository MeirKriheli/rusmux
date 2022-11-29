use crate::error::AppError;
use crate::project::Project;
use clap::crate_name;
use std::env;
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
            .args(&[
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
            .map(|line| {
                line.split(" ")
                    .skip(1)
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap()
            })
            .collect();

        if values.len() != 2 {
            return Err(AppError::Message(READ_ERROR.into()));
        }

        Ok(Self::new(values[0], values[1]))
    }

    /// Get the send key command as Vec<String>. `window_index` and `pane_index` should be zero
    /// base, the function will adjust them by the values of `base_index` and `pane_base_index`.
    fn get_send_keys_command<S: AsRef<str>>(
        &self,
        command: S,
        session_name: S,
        window_index: usize,
        pane_index: Option<usize>,
    ) -> Vec<String> {
        let formatted_pane_index =
            pane_index.map_or("".into(), |idx| format!(".{}", idx + self.pane_base_index));
        vec![
            TMUX_BIN.into(),
            "send-keys".into(),
            "-t".into(),
            format!(
                "{}:{}{}",
                session_name.as_ref(),
                window_index + self.base_index,
                formatted_pane_index
            ),
            command.as_ref().into(),
            "C-m".into(),
        ]
    }
}

/// Basic trait for commands ran by [TmuxProject]. Each command implements [fmt::Display] to
/// show the shell commands instead of executing them.
trait ProjectCommand: fmt::Display {
    fn run(&self) -> Result<(), AppError>;
}

/// Start the server and cd to the work directory if available
struct ServerStartCommand<'a> {
    project_name: &'a str,
    project_root: &'a Option<String>,
}

impl<'a> ProjectCommand for ServerStartCommand<'a> {
    fn run(&self) -> Result<(), AppError> {
        todo!()
    }
}

impl<'a> fmt::Display for ServerStartCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let shebang = env::var("SHELL").map(|x| format!("#!{}", x)).ok();
        let cd_command = match self.project_root {
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
            shebang.unwrap_or("".into()),
            crate_name!(),
            self.project_name,
            TMUX_BIN,
            cd_command
        )
    }
}

/// Run on_project_start commands
struct OnProjectStartCommand<'a> {
    on_project_start: &'a Option<Vec<String>>,
}

impl<'a> ProjectCommand for OnProjectStartCommand<'a> {
    fn run(&self) -> Result<(), AppError> {
        todo!()
    }
}

impl<'a> fmt::Display for OnProjectStartCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let commands = self
            .on_project_start
            .as_ref()
            .map_or(String::from(""), |v| v.join("\n"));
        write!(f, "# Run on_project_start command(s)\n{}", commands)
    }
}

/// Start the new tmux session, and cd again (for tmux < 1.9 compat)
struct SessionStartCommand<'a> {
    project_name: &'a str,
    first_window_name: Option<&'a str>,
}

impl<'a> ProjectCommand for SessionStartCommand<'a> {
    fn run(&self) -> Result<(), AppError> {
        todo!()
    }
}

impl<'a> fmt::Display for SessionStartCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let window_param = self
            .first_window_name
            .map_or("".into(), |n| format!(" -n {}", n));
        write!(
            f,
            "# Create new session and first window\n\
            TMUX= {} new-session -d -s {}{}",
            TMUX_BIN, self.project_name, window_param
        )
    }
}

/// Send Keys command
struct SendKeysCommand<'a> {
    tmux: &'a Tmux,
    command: &'a str,
    session_name: &'a str,
    window_index: usize,
    pane_index: Option<usize>,
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

    fn get_commands(&'a self) -> Vec<Box<dyn ProjectCommand + 'a>> {
        let project_name = &self.project.project_name;

        let first_window_name = self
            .project
            .windows
            .as_ref()
            .and_then(|windows| windows.first())
            .map(|w| w.name.as_str());

        vec![
            Box::new(ServerStartCommand {
                project_name,
                project_root: &self.project.project_root,
            }),
            Box::new(OnProjectStartCommand {
                on_project_start: &self.project.on_project_start,
            }),
            Box::new(SessionStartCommand {
                project_name,
                first_window_name,
            }),
        ]
    }
}

impl<'a> fmt::Display for TmuxProject<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let joined = self
            .get_commands()
            .into_iter()
            .map(|x| format!("{}\n", x))
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", joined)
    }
}
