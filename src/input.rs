use std::env;

#[derive(Debug)]
pub enum InputResult {
    VarError(env::VarError),
}

pub fn get_input(name: &str) -> Result<String, InputResult> {
    env::var(format!("INPUT_{}", name.to_uppercase())).map_err(|err| InputResult::VarError(err))
}

pub fn get_multiline_input(name: &str) -> Result<Vec<String>, InputResult> {
    get_input(name).map(|input| {
        input
            .split('\n')
            .filter(|v| *v != "")
            .map(|v| v.to_string())
            .collect()
    })
}

#[derive(Debug)]
pub enum BoolInputResult {
    VarError(env::VarError),
    TypeError,
}

pub fn get_bool_input(name: &str) -> Result<bool, BoolInputResult> {
    const TRUE_VALUE: [&str; 3] = ["true", "True", "TRUE"];
    const FALSE_VALUE: [&str; 3] = ["false", "False", "FALSE"];
    let val =
        get_input(name).map_err(|InputResult::VarError(err)| BoolInputResult::VarError(err))?;
    if TRUE_VALUE.contains(&val.as_str()) {
        return Ok(true);
    }
    if FALSE_VALUE.contains(&val.as_str()) {
        return Ok(false);
    }
    Err(BoolInputResult::TypeError)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_input() {
        env::set_var("INPUT_NAME", "value");
        assert_eq!("value".to_string(), get_input("name").unwrap());
    }

    #[test]
    fn test_get_input_none() {
        assert!(get_input("unknown").is_err())
    }

    #[test]
    fn test_get_bool_input() {
        env::set_var("INPUT_CI", "true");
        assert_eq!(true, get_bool_input("ci").unwrap());
    }

    #[test]
    fn test_get_multiline_input() {
        env::set_var("INPUT_LINES", "true\nfalse\n");
        assert_eq!(vec!["true", "false"], get_multiline_input("lines").unwrap());
    }
}
