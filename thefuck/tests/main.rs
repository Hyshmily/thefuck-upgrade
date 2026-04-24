use tempfile::tempdir;
use thefuck::corrector::Corrector;
use thefuck::types::{Command, Settings};
use thefuck::util;

#[test]
fn test_command_creation() {
    let cmd = Command::new("git status".to_string());
    assert_eq!(cmd.raw, "git status");
    assert_eq!(cmd.parts, vec!["git", "status"]);
}

#[test]
fn test_empty_command() {
    let cmd = Command::new("".to_string());
    assert!(cmd.is_empty());
}

#[test]
fn test_levenshtein_distance() {
    assert_eq!(util::levenshtein("git", "gti"), 2);
    assert_eq!(util::levenshtein("python", "pyhton"), 2);
    assert_eq!(util::levenshtein("commit", "comit"), 1);
    assert_eq!(util::levenshtein("", "abc"), 3);
    assert_eq!(util::levenshtein("abc", ""), 3);
    assert_eq!(util::levenshtein("", ""), 0);
}

#[test]
fn test_levenshtein_ratio() {
    let ratio = util::levenshtein_ratio("python", "python");
    assert!((ratio - 1.0).abs() < f64::EPSILON);
    assert!((util::levenshtein_ratio("", "") - 1.0).abs() < f64::EPSILON);
    assert!(util::levenshtein_ratio("abc", "def") < 0.5);
}

#[test]
fn test_git_typo_rule() {
    let command = Command::new("gti status".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);

    let matches = corrector.find_corrections();
    assert!(!matches.is_empty());
    assert_eq!(matches[0].corrected_command, "git status");
}

#[test]
fn test_git_typo_variants() {
    for typo in &["gitt", "gut"] {
        let command = Command::new(format!("{} status", typo));
        let settings = Settings::default();
        let corrector = Corrector::new(command, settings);

        let matches = corrector.find_corrections();
        assert!(!matches.is_empty());
        assert_eq!(matches[0].corrected_command, "git status");
    }
}

#[test]
fn test_git_subcommand_typo_rule() {
    let command = Command::new("git comit -m 'msg'".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);

    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command.contains("git commit -m")));
}

#[test]
fn test_git_checkout_to_switch_rule() {
    let command = Command::new("git checkout feature/login".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);

    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command == "git switch feature/login"));
}

#[test]
fn test_git_checkout_branch_create_to_switch_rule() {
    let command = Command::new("git checkout -b feature/api".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);

    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command == "git switch -c feature/api"));
}

#[test]
fn test_git_push_upstream_rule() {
    let command = Command::new("git push my-feature".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);

    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command == "git push --set-upstream origin my-feature"));
}

#[test]
fn test_git_force_with_lease_rule() {
    let command = Command::new("git push origin main --force".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);

    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command == "git push origin main --force-with-lease"));
}

#[test]
fn test_python_typo_rule() {
    let command = Command::new("pyhton -V".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);

    let matches = corrector.find_corrections();
    assert!(matches.iter().any(|m| m.corrected_command == "python -V"));
}

#[test]
fn test_python_typo_variants() {
    for typo in &["pyton", "puthon"] {
        let command = Command::new(format!("{} -V", typo));
        let settings = Settings::default();
        let corrector = Corrector::new(command, settings);

        let matches = corrector.find_corrections();
        assert!(matches.iter().any(|m| m.corrected_command == "python -V"));
    }
}

#[test]
fn test_python_pip_module_rule() {
    let command = Command::new("pip install requests".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);

    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command == "python -m pip install requests"));
}

#[test]
fn test_python_pip_to_uv_rule() {
    let command = Command::new("pip install requests".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);

    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command == "uv pip install requests"));
}

#[test]
fn test_python_pip3_is_modernized() {
    let command = Command::new("pip3 install requests".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);

    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command == "uv pip3 install requests"));
}

#[test]
fn test_docker_compose_rule() {
    let command = Command::new("docker-compose up -d".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);

    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command == "docker compose up -d"));
}

#[test]
fn test_docker_images_to_image_ls_rule() {
    let command = Command::new("docker images -a".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);

    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command == "docker image ls -a"));
}

#[test]
fn test_docker_ps_to_container_ls_rule() {
    let command = Command::new("docker ps -a".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);

    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command == "docker container ls -a"));
}

#[test]
fn test_sudo_missing_rule() {
    let command = Command::new("apt-get install vim".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);

    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command == "sudo apt-get install vim"));
}

#[test]
fn test_sudo_missing_with_apt() {
    let command = Command::new("apt install vim".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);

    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command == "sudo apt install vim"));
}

#[test]
fn test_sudo_missing_with_systemctl() {
    let command = Command::new("systemctl start nginx".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);

    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command == "sudo systemctl start nginx"));
}

#[test]
fn test_sudo_not_added_for_irrelevant_commands() {
    let command = Command::new("ls -la".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);

    let matches = corrector.find_corrections();
    assert!(matches.iter().all(|m| !m.corrected_command.starts_with("sudo ")));
}

#[test]
fn test_no_correction_for_correct_git() {
    let command = Command::new("git status".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);

    let matches = corrector.find_corrections();
    // git status shouldn't match any rule
    assert!(!matches
        .iter()
        .any(|m| m.corrected_command == "git status"));
}

#[test]
fn test_cd_correction_rule() {
    let temp = tempdir().expect("tempdir");
    std::fs::create_dir_all(temp.path().join("project")).expect("create dir");

    let original_dir = std::env::current_dir().expect("cwd");
    std::env::set_current_dir(temp.path()).expect("set cwd");

    let command = Command::new("cd proje".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);
    let matches = corrector.find_corrections();

    std::env::set_current_dir(original_dir).expect("restore cwd");

    assert!(matches.iter().any(|m| m.corrected_command == "cd project"));
}

#[test]
fn test_exclude_rules() {
    let command = Command::new("gti status".to_string());
    let mut settings = Settings::default();
    settings.exclude_rules = vec!["git_command".to_string()];
    let corrector = Corrector::new(command, settings);

    let matches = corrector.find_corrections();
    assert!(matches.iter().all(|m| m.rule != "git_command"));
}

#[test]
fn test_num_close_matches_limit() {
    let command = Command::new("pip install requests".to_string());
    let mut settings = Settings::default();
    settings.num_close_matches = 1;
    let corrector = Corrector::new(command, settings);

    let matches = corrector.find_corrections();
    assert!(matches.len() <= 1);
}
