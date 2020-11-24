use crate::error::AppError;
use crate::project::Project;
use std::env;
use std::fmt;
use std::process::Command;

const TMUX: &str = "tmux";
const READ_ERROR: &str = "Cannot get tmux config options";

#[derive(Debug)]
struct TmuxOptions {
    base_index: usize,
    pane_base_index: usize,
}

impl TmuxOptions {
    fn new(base_index: usize, pane_base_index: usize) -> Self {
        Self {
            base_index,
            pane_base_index,
        }
    }

    fn new_from_config() -> Result<Self, AppError> {
        let output = Command::new(TMUX)
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
}

#[derive(Debug)]
pub struct TmuxProject<'a> {
    tmux_options: TmuxOptions,
    project: &'a Project,
}

impl<'a> TmuxProject<'a> {
    pub fn new(project: &'a Project) -> Result<Self, AppError> {
        let tmux_options = TmuxOptions::new_from_config()?;
        Ok(TmuxProject {
            tmux_options,
            project,
        })
    }
}
impl<'a> fmt::Display for TmuxProject<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let shebang = env::var("SHELL").map(|x| format!("#!{}\n#", x)).ok();
        let first_window_name_param = match self.project.windows.as_ref() {
            Some(windows) => windows
                .first()
                .map(|w| format!(" -s {}", w.name))
                .unwrap_or("".into()),
            _ => "".into(),
        };

        let commands = vec![
            shebang,
            Some(format!("# {} rusmux project\n", self.project.project_name)),
            Some(format!("{} start server\n", TMUX)),
            self.project.project_root.as_ref().map(|x| format!("cd {}\n", x)),
            self.project.on_project_start
                .as_ref()
                .map(|x| format!("# Run on_project_start command(s)\n{}\n", x.join("\n"))),
            Some(format!(
                "# Create new session and first window\nTMUX= {} new-session -d -s {}{}",
                TMUX, self.project.project_name, first_window_name_param
            )),
        ];

        let joined = commands
            .into_iter()
            .filter_map(|c| c)
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", joined)
    }
}
