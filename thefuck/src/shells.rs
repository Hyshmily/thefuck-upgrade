use std::env;

pub struct ShellInfo {
    pub name: String,
}

pub fn shell() -> ShellInfo {
    let shell_name = detect_shell();
    ShellInfo { name: shell_name }
}

fn detect_shell() -> String {
    if let Ok(shell) = env::var("SHELL") {
        return shell;
    }

    if cfg!(windows) {
        if let Ok(ps_module_path) = env::var("PSModulePath") {
            if !ps_module_path.is_empty() {
                return "powershell".to_string();
            }
        }
    }

    env::var("SHELL_TYPE").unwrap_or_else(|_| "unknown".to_string())
}
