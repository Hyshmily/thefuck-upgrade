use tempfile::tempdir;
use thefuck::corrector::{levenshtein, levenshtein_ratio, Corrector};
use thefuck::types::{Command, Settings};

#[test]
fn test_command_creation() {
    let cmd = Command::new("git status".to_string());
    assert_eq!(cmd.raw, "git status");
    assert_eq!(cmd.parts, vec!["git", "status"]);
}

#[test]
fn test_levenshtein_distance() {
    assert_eq!(levenshtein("git", "gti"), 2);
    assert_eq!(levenshtein("python", "pyhton"), 2);
    assert_eq!(levenshtein("commit", "comit"), 1);
}

#[test]
fn test_levenshtein_ratio() {
    let ratio = levenshtein_ratio("python", "python");
    assert!((ratio - 1.0).abs() < f64::EPSILON);
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
fn test_python_typo_rule() {
    let command = Command::new("pyhton -V".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);

    let matches = corrector.find_corrections();
    assert!(matches.iter().any(|m| m.corrected_command == "python -V"));
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
