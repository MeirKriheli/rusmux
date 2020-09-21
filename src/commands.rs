use crate::error::AppError;
use crate::project::Project;

use std::env;

pub trait Command {
    fn execute(&self) -> Result<(), AppError>;
    fn debug(&self) -> String;
}

pub struct SessionStartCommand<'a> {
    project: &'a Project,
}

impl<'a> SessionStartCommand<'a> {
    pub fn new(project: &'a Project) -> Self {
        Self { project }
    }
    ///
    /// Get the shell shebang, e.g.: "#!/bin/zsh"
    fn get_shell_shebang(&self) -> Option<String> {
        let shell = env::var("SHELL");

        match shell {
            Ok(shell) => Some(format!("#!{}", shell)),
            _ => None,
        }
    }

    fn get_root_command(&self) -> Option<String> {
        self.project
            .project_root
            .as_ref()
            .map(|dir| format!("cd {}", dir))
    }

    fn get_project_start_commands(&self) -> Option<String> {
        self.project.on_project_start.as_ref().map(|commands| {
            let joined = commands.join("\n");
            format!("\n# Run on_project_start command(s)\n{}", joined)
        })
    }
}

impl<'a> Command for SessionStartCommand<'a> {
    fn execute(&self) -> Result<(), AppError> {
        Ok(())
    }

    fn debug(&self) -> String {
        let result = vec![
            self.get_shell_shebang(),
            Some("\ntmux start-server\n".into()),
            self.get_root_command(),
            self.get_project_start_commands(),
        ];

        result
            .into_iter()
            .filter_map(|c| c)
            .collect::<Vec<String>>()
            .join("\n")
    }
}
