use std::fmt::Display;

pub struct Property<'a>(pub &'a str, pub &'a str);

impl<'a> Display for Property<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}={}", self.0, escape_property(self.1)))
    }
}

pub struct Properties<'a>(pub Vec<Property<'a>>);

impl<'a> From<Vec<(&'a str, &'a str)>> for Properties<'a> {
    fn from(value: Vec<(&'a str, &'a str)>) -> Self {
        Properties(value.into_iter().map(|(k, v)| Property(k, v)).collect())
    }
}

impl<'a> From<&[(&'a str, &'a str)]> for Properties<'a> {
    fn from(value: &[(&'a str, &'a str)]) -> Self {
        Properties(value.iter().map(|(k, v)| Property(k, v)).collect())
    }
}

impl<'a, const N: usize> From<[(&'a str, &'a str); N]> for Properties<'a> {
    fn from(value: [(&'a str, &'a str); N]) -> Self {
        Properties(value.into_iter().map(|(k, v)| Property(k, v)).collect())
    }
}

impl<'a> Display for Properties<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(",")
                .as_str(),
        )
    }
}

pub struct Command<'a> {
    pub command: &'a str,
    pub value: &'a str,
    pub properties: Option<Properties<'a>>,
}

impl<'a> Display for Command<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.properties {
            Some(properties) => f.write_fmt(format_args!(
                "::{} {}::{}",
                self.command,
                properties,
                escape_data(self.value)
            )),
            None => f.write_fmt(format_args!(
                "::{}::{}",
                self.command,
                escape_data(self.value)
            )),
        }
    }
}

#[derive(Default)]
pub struct CommandWithProperties<'a> {
    pub command: &'a str,
    pub value: &'a str,
    pub title: Option<&'a str>,
    pub file: Option<&'a str>,
    pub col: Option<usize>,
    pub end_column: Option<usize>,
    pub line: Option<usize>,
    pub end_line: Option<usize>,
}

impl<'a> Display for CommandWithProperties<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let col = self.col.map(|v| v.to_string());
        let end_column = self.end_column.map(|v| v.to_string());
        let line = self.line.map(|v| v.to_string());
        let end_line = self.line.map(|v| v.to_string());
        let params: Vec<(&str, &str)> = vec![
            ("title", self.title),
            ("file", self.file),
            ("col", col.as_deref()),
            ("endColumn", end_column.as_deref()),
            ("line", line.as_deref()),
            ("endLine", end_line.as_deref()),
        ]
        .into_iter()
        .filter_map(|(k, v)| v.map(|v| (k, v)))
        .collect();

        Command {
            command: self.command,
            value: self.value,
            properties: Some(params.into()),
        }
        .fmt(f)
    }
}

pub fn escape_data<T: AsRef<str>>(s: T) -> String {
    s.as_ref()
        .replace('%', "%25")
        .replace('\r', "%0D")
        .replace('\n', "%0A")
}

pub fn escape_property<T: AsRef<str>>(s: T) -> String {
    s.as_ref()
        .replace('%', "%25")
        .replace('\r', "%0D")
        .replace('\n', "%0A")
        .replace(':', "%3A")
        .replace(',', "%2C")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message() {
        let message = Command {
            command: "command",
            properties: None,
            value: "some message",
        };

        assert_eq!("::command::some message", message.to_string());
    }

    #[test]
    fn test_message_with_property() {
        let message = Command {
            command: "command",
            properties: Some(Properties(vec![Property("title", "value")])),
            value: "some message",
        };

        assert_eq!("::command title=value::some message", message.to_string());
    }

    #[test]
    fn test_message_with_properties() {
        let message = Command {
            command: "command",
            properties: Some(Properties(vec![
                Property("title", "value"),
                Property("line", "1"),
            ])),
            value: "some message",
        };

        assert_eq!(
            "::command title=value,line=1::some message",
            message.to_string()
        );
    }
}
