use crate::types::{Command, MatchResult};
use crate::util;

pub fn sudo_missing_rule(command: &Command) -> Option<MatchResult> {
    if command.parts.is_empty() {
        return None;
    }

    if command.parts[0] == "sudo" {
        return None;
    }

    let suspicious = matches!(
        command.parts[0].as_str(),
        "apt-get" | "apt" | "dnf" | "pacman" | "systemctl" | "yum" | "zypper" | "snap" | "flatpak"
    );

    // "make install" typically needs sudo
    let is_make_install =
        command.parts.len() >= 2 && command.parts[0] == "make" && command.parts[1] == "install";

    // "npm install -g" typically needs sudo
    let is_npm_global = command.parts.len() >= 3
        && command.parts[0] == "npm"
        && command.parts[1] == "install"
        && command.parts.iter().any(|p| p == "-g" || p == "--global");

    if !suspicious && !is_make_install && !is_npm_global {
        return None;
    }

    Some(MatchResult {
        rule: "sudo_missing",
        corrected_command: format!("sudo {}", command.raw),
        similarity: util::SIMILARITY_SUDO,
    })
}
