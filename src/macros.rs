#[macro_export]
macro_rules! info {
    ($value:expr) => {
        println!("{}", $value);
    };
    ($($arg:tt)*) => {
        $crate::info!(format!($($arg)*));
    };
}

#[macro_export]
macro_rules! issue_command {
    ($command:ident, $value:expr) => {
        println!(
            "{}",
            $crate::Command {
                command: stringify!($command),
                value: $value.as_ref() as &str,
                properties: None,
            }
        );
    };
    ($command:ident, $value:expr, $($key:ident: $val:expr),+) => {
        println!("{}", $crate::Command {
            command: stringify!($command),
            value: $value.as_ref() as &str,
            properties: Some([$((stringify!($key), $val.as_ref() as &str)),+].into()),
        });
    };
}

#[macro_export]
macro_rules! issue_command_with_properties {
    ($command:ident, $value:expr, $key1:ident: $val1:expr, $key2:ident: $val2:expr, $key3:ident: $val3:expr, $key4:ident: $val4:expr, $key5:ident: $val5:expr, $key6:ident: $val6:expr) => {
        println!("{}", $crate::CommandWithProperties {
            command: stringify!($command),
            value: $value.as_ref() as &str,
            $key1: Some($val1),
            $key2: Some($val2),
            $key3: Some($val3),
            $key4: Some($val4),
            $key5: Some($val5),
            $key6: Some($val6),
        });
    };
    ($command:ident, $value:expr, $($key:ident: $val:expr),*) => {
        println!("{}", $crate::CommandWithProperties {
            command: stringify!($command),
            value: $value.as_ref() as &str,
            $($key: Some($val)),+,
            ..Default::default()
        });
    };
}

#[macro_export]
macro_rules! debug {
    ($value:expr) => {
        issue_command!(debug, $value);
    };
    ($($arg:tt)*) => {
        $crate::debug!(format!($($arg)*));
    };
}

#[macro_export]
macro_rules! error {
    ($value:expr) => {
        issue_command!(error, $value);
    };
    ($value:expr, $($key:ident: $val:expr),+) => {
        issue_command_with_properties!(error, $value, $($key: $val),+);
    };
}

#[macro_export]
macro_rules! warn {
    ($value:expr) => {
        issue_command!(warning, $value);
    };
    ($value:expr, $($key:ident: $val:expr),+) => {
        issue_command_with_properties!(warning, $value, $($key: $val),+);
    };
}

#[macro_export]
macro_rules! notice {
    ($value:expr) => {
        issue_command!(notice, $value);
    };
    ($value:expr, $($key:ident: $val:expr),+) => {
        issue_command_with_properties!(notice, $value, $($key: $val),+);
    };
}

#[macro_export]
macro_rules! group {
    ($title:expr) => {
        let __group = $crate::Group::new($title.as_ref() as &str);
        let __group_ender = $crate::GroupEnder::new(&__group);
    };
    ($($arg:tt)*) => {
        $crate::group!(format!($($arg)*));
    };
}

#[macro_export]
macro_rules! stop_commands {
    () => {
        let __stop_commands = $crate::StopCommands::gen();
        let __stop_commands_ender = $crate::StopCommandsEnder::new(&__stop_commands);
    };
    ($token:expr) => {
        let __stop_commands = $crate::StopCommands::new($token.as_ref() as &str);
        let __stop_commands_ender = $crate::StopCommandsEnder::new(&__stop_commands);
    };
    ($($arg:tt)*) => {
        $crate::stop_commands!(format!($($arg)*));
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_commands() {
        issue_command!(cmd, format!("val"), k: format!("prop"));
        issue_command!(cmd, "val", k: "prop");
        info!("message");
        debug!("message");
        debug!("message {}", "debug");
        error!("message");
        error!("message", title: "title", file: "file.rs");
        error!("message", title: "title", file: "file", col: 1, end_column: 1, line: 1);
        error!("message", title: "title", file: "file", col: 1, end_column: 1, line: 1, end_line: 1);
        warn!("message");
        warn!("message", title: "title");
        warn!("message", title: "title", file: "file", col: 1, end_column: 1, line: 1, end_line: 1);
        notice!("message");
        notice!("message", title: "title");
        notice!("message", title: "title", file: "file", col: 1, end_column: 1, line: 1, end_line: 1);
    }

    #[test]
    fn test_group() {
        {
            group!("group {}", 1);
        }
        {
            group!(format!("group {}", 2));
            println!("log");
        }
        group!("group last");
        println!("log");
    }

    #[test]
    fn test_stop_commands() {
        {
            let token = format!("token{}", 1);
            stop_commands!(token);
            println!("log");
        }
        {
            stop_commands!();
            println!("log");
        }
        stop_commands!("token");
        println!("log");
    }
}
