use std::env;

pub struct ShellInfo {
    pub name: String,
    pub version: String,
}

pub fn shell() -> ShellInfo {
    let shell_name = env::var("SHELL")
        .or_else(|_| env::var("SHELL_TYPE"))
        .unwrap_or_else(|_| "unknown".to_string());

    ShellInfo {
        name: shell_name,
        version: "unknown".to_string(),
    }
}
