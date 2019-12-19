#[derive(Debug)]
pub struct Cmd<'a> {
    command: Vec<&'a str>,
    reset_tmux_env: bool,
    as_debug: Option<&'a str>,
}

impl<'a> Cmd<'a> {
    pub fn new(command: Vec<&'a str>, reset_tmux_env: bool, as_debug: Option<&'a str>) -> Self {
        Cmd {
            command: command,
            reset_tmux_env: reset_tmux_env,
            as_debug: as_debug,
        }
    }

    pub fn execute(&self) {}
}
