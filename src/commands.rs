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
}

impl<'a> Command for SessionStartCommand<'a> {
    fn execute(&self) -> Result<(), AppError> {
        Ok(())
    }

    fn debug(&self) -> String {
        let mut result = vec![
            self.get_shell_shebang(),
            Some("\ntmux start-server\n".into()),
            self.project.project_root.as_ref().map(|dir| format!("cd {}", dir)),
        ];

        result.into_iter().filter_map(|c| c).collect::<Vec<String>>().join("\n")
    }
}
