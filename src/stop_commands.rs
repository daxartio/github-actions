fn start_stop_commands(token: &str) {
    println!(
        "{}",
        crate::Command {
            command: "stop-commands",
            value: token,
            properties: None
        }
    );
}

fn end_stop_commands(token: &str) {
    println!(
        "{}",
        crate::Command {
            command: token,
            value: "",
            properties: None
        }
    );
}

pub struct StopCommands<T: AsRef<str>> {
    token: T,
}

impl<T: AsRef<str>> StopCommands<T> {
    pub fn new(token: T) -> Self {
        start_stop_commands(token.as_ref());
        Self { token }
    }

    pub fn end(&self) {
        end_stop_commands(self.token.as_ref());
    }
}

impl StopCommands<String> {
    pub fn gen() -> Self {
        StopCommands::<String>::new(crate::rand::random_id())
    }
}

pub struct StopCommandsEnder<'a, T: AsRef<str>> {
    parent: &'a StopCommands<T>,
}

impl<'a, T: AsRef<str>> StopCommandsEnder<'a, T> {
    pub fn new(stop_commands: &'a StopCommands<T>) -> Self {
        Self {
            parent: stop_commands,
        }
    }
}

impl<T: AsRef<str>> Drop for StopCommandsEnder<'_, T> {
    fn drop(&mut self) {
        self.parent.end();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        {
            let stop_cmds = StopCommands::new("token1");
            let _stop_cmds_ender = StopCommandsEnder::new(&stop_cmds);
        }
        {
            let _stop_cmds = StopCommands::new("token2");
            // it doesn't end
        }
    }
}
