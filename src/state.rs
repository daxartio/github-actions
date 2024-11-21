use std::env;

#[derive(Debug)]
pub enum StateResult {
    VarError(env::VarError),
}

pub fn get_state(name: &str) -> Result<String, StateResult> {
    env::var(format!("STATE_{}", name.to_uppercase())).map_err(|err| StateResult::VarError(err))
}
