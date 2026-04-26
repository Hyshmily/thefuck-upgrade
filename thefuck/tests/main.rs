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
    assert!(cmd.parts.is_empty());
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
fn test_mvn_typo_rule() {
    let command = Command::new("mvnm clean".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);

    let matches = corrector.find_corrections();
    assert!(matches.iter().any(|m| m.corrected_command == "mvn clean"));
}

#[test]
fn test_mvn_typo_variants() {
    for typo in &["mnv"] {
        let command = Command::new(format!("{} clean", typo));
        let settings = Settings::default();
        let corrector = Corrector::new(command, settings);

        let matches = corrector.find_corrections();
        assert!(matches.iter().any(|m| m.corrected_command == "mvn clean"));
    }
}

#[test]
fn test_mvn_subcommand_typo_rule() {
    let command = Command::new("mvn clea".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);

    let matches = corrector.find_corrections();
    assert!(matches.iter().any(|m| m.corrected_command == "mvn clean"));
}

#[test]
fn test_mvn_subcommand_typo_variants() {
    for (typo, expected) in &[
        ("complie", "compile"),
        ("instlal", "install"),
        ("depoy", "deploy"),
    ] {
        let command = Command::new(format!("mvn {}", typo));
        let settings = Settings::default();
        let corrector = Corrector::new(command, settings);

        let matches = corrector.find_corrections();
        assert!(matches
            .iter()
            .any(|m| m.corrected_command == format!("mvn {}", expected)));
    }
}

#[test]
fn test_mvn_no_correction_for_correct() {
    let command = Command::new("mvn clean install".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);

    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .all(|m| m.rule != "mvn_subcommand_typo" && m.rule != "mvn_multiphase_typo"));
}

#[test]
fn test_mvn_multiphase_typo_at_position_2() {
    let command = Command::new("mvn clean instlal".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);

    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command == "mvn clean install"));
}

#[test]
fn test_mvn_multiphase_typo_fuzzy() {
    let command = Command::new("mvn verfy".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);

    let matches = corrector.find_corrections();
    assert!(matches.iter().any(|m| m.corrected_command == "mvn verify"));
}

#[test]
fn test_mvn_multiphase_skips_flags() {
    let command = Command::new("mvn -DskipTests clean".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);

    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .all(|m| m.rule != "mvn_subcommand_typo" && m.rule != "mvn_multiphase_typo"));
}

#[test]
fn test_mvn_validate_typo() {
    let command = Command::new("mvn validae".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);

    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command == "mvn validate"));
}

#[test]
fn test_mvn_multiphase_no_false_positive_on_correct_phases() {
    let command = Command::new("mvn clean compile test".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);

    let matches = corrector.find_corrections();
    assert!(matches.iter().all(|m| m.rule != "mvn_multiphase_typo"));
}

#[test]
fn test_mvn_subcommand_fuzzy_unknown_typo() {
    let command = Command::new("mvn clan".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);

    let matches = corrector.find_corrections();
    assert!(matches.iter().any(|m| m.corrected_command == "mvn clean"));
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
    assert!(matches
        .iter()
        .all(|m| !m.corrected_command.starts_with("sudo ")));
}

#[test]
fn test_no_correction_for_correct_git() {
    let command = Command::new("git status".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);

    let matches = corrector.find_corrections();
    // git status shouldn't match any rule
    assert!(!matches.iter().any(|m| m.corrected_command == "git status"));
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

// -- Common typo tests --

#[test]
fn test_common_typo_sl_to_ls() {
    let command = Command::new("sl -la".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);
    let matches = corrector.find_corrections();
    assert!(matches.iter().any(|m| m.corrected_command == "ls -la"));
}

#[test]
fn test_common_typo_gerp_to_grep() {
    let command = Command::new("gerp pattern file".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);
    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command == "grep pattern file"));
}

#[test]
fn test_common_typo_variants() {
    let cases = [
        ("mkae", "make"),
        ("ehco", "echo"),
        ("chomd", "chmod"),
        ("chwon", "chown"),
        ("clera", "clear"),
        ("hsitory", "history"),
        ("exir", "exit"),
        ("tuch", "touch"),
        ("mrdir", "mkdir"),
        ("vom", "vim"),
        ("fid", "find"),
        ("pc", "cp"),
        ("cta", "cat"),
        ("mvv", "mv"),
        ("rmr", "rm"),
    ];
    for (typo, expected) in &cases {
        let command = Command::new(format!("{} arg", typo));
        let settings = Settings::default();
        let corrector = Corrector::new(command, settings);
        let matches = corrector.find_corrections();
        assert!(
            matches
                .iter()
                .any(|m| m.corrected_command == format!("{} arg", expected)),
            "expected {expected} for typo {typo}"
        );
    }
}

#[test]
fn test_no_correction_for_correct_common_commands() {
    let command = Command::new("ls -la".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);
    let matches = corrector.find_corrections();
    assert!(matches.iter().all(|m| m.rule != "common_typo"));
}

// -- npm/yarn tests --

#[test]
fn test_npm_typo() {
    let command = Command::new("npn install express".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);
    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command == "npm install express"));
}

#[test]
fn test_npm_subcommand_typo() {
    let command = Command::new("npm isntall express".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);
    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command == "npm install express"));
}

#[test]
fn test_yarn_typo() {
    let command = Command::new("yrn add express".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);
    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command == "yarn add express"));
}

#[test]
fn test_yarn_subcommand_typo() {
    let command = Command::new("yarn isntall".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);
    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command == "yarn install"));
}

#[test]
fn test_npm_subcommand_typo_variants() {
    let cases = [
        ("unistall", "uninstall"),
        ("udpate", "update"),
        ("iniit", "init"),
        ("satrt", "start"),
        ("tets", "test"),
        ("rnu", "run"),
        ("bulid", "build"),
        ("publis", "publish"),
    ];
    for (typo, expected) in &cases {
        let command = Command::new(format!("npm {} arg", typo));
        let settings = Settings::default();
        let corrector = Corrector::new(command, settings);
        let matches = corrector.find_corrections();
        assert!(
            matches
                .iter()
                .any(|m| m.corrected_command == format!("npm {} arg", expected)),
            "expected {expected} for typo {typo}"
        );
    }
}

// -- Cargo tests --

#[test]
fn test_cargo_typo() {
    let command = Command::new("carg build".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);
    let matches = corrector.find_corrections();
    assert!(matches.iter().any(|m| m.corrected_command == "cargo build"));
}

#[test]
fn test_cargo_subcommand_typo() {
    let command = Command::new("cargo buid".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);
    let matches = corrector.find_corrections();
    assert!(matches.iter().any(|m| m.corrected_command == "cargo build"));
}

#[test]
fn test_cargo_subcommand_typo_variants() {
    let cases = [
        ("tset", "test"),
        ("rnu", "run"),
        ("chekc", "check"),
        ("clipp", "clippy"),
        ("frmt", "fmt"),
        ("clea", "clean"),
        ("isntall", "install"),
    ];
    for (typo, expected) in &cases {
        let command = Command::new(format!("cargo {}", typo));
        let settings = Settings::default();
        let corrector = Corrector::new(command, settings);
        let matches = corrector.find_corrections();
        assert!(
            matches
                .iter()
                .any(|m| m.corrected_command == format!("cargo {}", expected)),
            "expected {expected} for typo {typo}"
        );
    }
}

// -- Go tests --

#[test]
fn test_go_typo() {
    let command = Command::new("og build".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);
    let matches = corrector.find_corrections();
    assert!(matches.iter().any(|m| m.corrected_command == "go build"));
}

#[test]
fn test_go_subcommand_typo() {
    let command = Command::new("go buid".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);
    let matches = corrector.find_corrections();
    assert!(matches.iter().any(|m| m.corrected_command == "go build"));
}

#[test]
fn test_go_subcommand_typo_variants() {
    let cases = [
        ("tset", "test"),
        ("rnu", "run"),
        ("frmt", "fmt"),
        ("mdo", "mod"),
        ("isntall", "install"),
        ("vte", "vet"),
    ];
    for (typo, expected) in &cases {
        let command = Command::new(format!("go {}", typo));
        let settings = Settings::default();
        let corrector = Corrector::new(command, settings);
        let matches = corrector.find_corrections();
        assert!(
            matches
                .iter()
                .any(|m| m.corrected_command == format!("go {}", expected)),
            "expected {expected} for typo {typo}"
        );
    }
}

// -- pip tests --

#[test]
fn test_pip_subcommand_typo() {
    let command = Command::new("pip isntall requests".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);
    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command == "pip install requests"));
}

#[test]
fn test_pip_subcommand_typo_variants() {
    let cases = [
        ("unistall", "uninstall"),
        ("freze", "freeze"),
        ("lits", "list"),
        ("shwo", "show"),
        ("chekc", "check"),
    ];
    for (typo, expected) in &cases {
        let command = Command::new(format!("pip {}", typo));
        let settings = Settings::default();
        let corrector = Corrector::new(command, settings);
        let matches = corrector.find_corrections();
        assert!(
            matches
                .iter()
                .any(|m| m.corrected_command == format!("pip {}", expected)),
            "expected {expected} for typo {typo}"
        );
    }
}

// -- Brew tests --

#[test]
fn test_brew_typo() {
    let command = Command::new("berw install node".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);
    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command == "brew install node"));
}

#[test]
fn test_brew_subcommand_typo() {
    let command = Command::new("brew isntall node".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);
    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command == "brew install node"));
}

#[test]
fn test_brew_subcommand_typo_variants() {
    let cases = [
        ("unistall", "uninstall"),
        ("udpate", "update"),
        ("upgarde", "upgrade"),
        ("docto", "doctor"),
    ];
    for (typo, expected) in &cases {
        let command = Command::new(format!("brew {}", typo));
        let settings = Settings::default();
        let corrector = Corrector::new(command, settings);
        let matches = corrector.find_corrections();
        assert!(
            matches
                .iter()
                .any(|m| m.corrected_command == format!("brew {}", expected)),
            "expected {expected} for typo {typo}"
        );
    }
}

// -- apt tests --

#[test]
fn test_apt_typo() {
    let command = Command::new("atp install vim".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);
    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command == "apt install vim"));
}

#[test]
fn test_apt_subcommand_typo() {
    let command = Command::new("apt isntall vim".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);
    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command == "apt install vim"));
}

#[test]
fn test_apt_get_to_apt() {
    let command = Command::new("apt-get install vim".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);
    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command == "apt install vim"));
}

#[test]
fn test_apt_subcommand_typo_variants() {
    let cases = [
        ("udpate", "update"),
        ("upgarde", "upgrade"),
        ("romve", "remove"),
    ];
    for (typo, expected) in &cases {
        let command = Command::new(format!("apt {}", typo));
        let settings = Settings::default();
        let corrector = Corrector::new(command, settings);
        let matches = corrector.find_corrections();
        assert!(
            matches
                .iter()
                .any(|m| m.corrected_command == format!("apt {}", expected)),
            "expected {expected} for typo {typo}"
        );
    }
}

// -- systemctl tests --

#[test]
fn test_systemctl_typo() {
    let command = Command::new("systemclt start nginx".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);
    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command == "systemctl start nginx"));
}

#[test]
fn test_systemctl_subcommand_typo() {
    let command = Command::new("systemctl satrt nginx".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);
    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command == "systemctl start nginx"));
}

#[test]
fn test_systemctl_subcommand_typo_variants() {
    let cases = [
        ("stpo", "stop"),
        ("restar", "restart"),
        ("enabel", "enable"),
        ("dsiable", "disable"),
        ("statsu", "status"),
    ];
    for (typo, expected) in &cases {
        let command = Command::new(format!("systemctl {} nginx", typo));
        let settings = Settings::default();
        let corrector = Corrector::new(command, settings);
        let matches = corrector.find_corrections();
        assert!(
            matches
                .iter()
                .any(|m| m.corrected_command == format!("systemctl {} nginx", expected)),
            "expected {expected} for typo {typo}"
        );
    }
}

// -- kubectl tests --

#[test]
fn test_kubectl_typo() {
    let command = Command::new("kubctl get pods".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);
    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command == "kubectl get pods"));
}

#[test]
fn test_kubectl_subcommand_typo() {
    let command = Command::new("kubectl aplpy -f pod.yaml".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);
    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command == "kubectl apply -f pod.yaml"));
}

#[test]
fn test_kubectl_subcommand_typo_variants() {
    let cases = [
        ("describ", "describe"),
        ("delte", "delete"),
        ("cretate", "create"),
        ("gett", "get"),
        ("exc", "exec"),
    ];
    for (typo, expected) in &cases {
        let command = Command::new(format!("kubectl {} resource", typo));
        let settings = Settings::default();
        let corrector = Corrector::new(command, settings);
        let matches = corrector.find_corrections();
        assert!(
            matches
                .iter()
                .any(|m| m.corrected_command == format!("kubectl {} resource", expected)),
            "expected {expected} for typo {typo}"
        );
    }
}

// -- Terraform tests --

#[test]
fn test_terraform_typo() {
    let command = Command::new("terrafrom init".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);
    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command == "terraform init"));
}

#[test]
fn test_terraform_subcommand_typo() {
    let command = Command::new("terraform aplpy".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);
    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command == "terraform apply"));
}

#[test]
fn test_terraform_subcommand_typo_variants() {
    let cases = [
        ("plna", "plan"),
        ("int", "init"),
        ("destory", "destroy"),
        ("outptu", "output"),
    ];
    for (typo, expected) in &cases {
        let command = Command::new(format!("terraform {}", typo));
        let settings = Settings::default();
        let corrector = Corrector::new(command, settings);
        let matches = corrector.find_corrections();
        assert!(
            matches
                .iter()
                .any(|m| m.corrected_command == format!("terraform {}", expected)),
            "expected {expected} for typo {typo}"
        );
    }
}

// -- Conda tests --

#[test]
fn test_conda_typo() {
    let command = Command::new("cnda install numpy".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);
    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command == "conda install numpy"));
}

#[test]
fn test_conda_subcommand_typo() {
    let command = Command::new("conda isntall numpy".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);
    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command == "conda install numpy"));
}

#[test]
fn test_conda_subcommand_typo_variants() {
    let cases = [
        ("romve", "remove"),
        ("lits", "list"),
        ("cretate", "create"),
        ("acitvate", "activate"),
        ("decativate", "deactivate"),
    ];
    for (typo, expected) in &cases {
        let command = Command::new(format!("conda {} env", typo));
        let settings = Settings::default();
        let corrector = Corrector::new(command, settings);
        let matches = corrector.find_corrections();
        assert!(
            matches
                .iter()
                .any(|m| m.corrected_command == format!("conda {} env", expected)),
            "expected {expected} for typo {typo}"
        );
    }
}

// -- Missing space / wrong hyphen tests --

#[test]
fn test_missing_space_cd_dotdot() {
    let command = Command::new("cd..".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);
    let matches = corrector.find_corrections();
    assert!(matches.iter().any(|m| m.corrected_command == "cd .."));
}

#[test]
fn test_missing_space_cd_dir() {
    let command = Command::new("cdDownloads".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);
    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command == "cd Downloads"));
}

#[test]
fn test_wrong_hyphen_git_log() {
    let command = Command::new("git-log".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);
    let matches = corrector.find_corrections();
    assert!(matches.iter().any(|m| m.corrected_command == "git log"));
}

#[test]
fn test_wrong_hyphen_npm_install() {
    let command = Command::new("npm-install".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);
    let matches = corrector.find_corrections();
    assert!(matches.iter().any(|m| m.corrected_command == "npm install"));
}

#[test]
fn test_wrong_hyphen_variants() {
    let cases = [
        ("docker-compose", "docker compose"),
        ("apt-update", "apt update"),
        ("brew-install", "brew install"),
        ("cargo-build", "cargo build"),
        ("go-test", "go test"),
    ];
    for (typo, expected) in &cases {
        let command = Command::new(typo.to_string());
        let settings = Settings::default();
        let corrector = Corrector::new(command, settings);
        let matches = corrector.find_corrections();
        assert!(
            matches.iter().any(|m| m.corrected_command == *expected),
            "expected {expected} for typo {typo}"
        );
    }
}

// -- Extended sudo tests --

#[test]
fn test_sudo_missing_with_yum() {
    let command = Command::new("yum install vim".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);
    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command == "sudo yum install vim"));
}

#[test]
fn test_sudo_missing_with_make_install() {
    let command = Command::new("make install".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);
    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command == "sudo make install"));
}

#[test]
fn test_sudo_missing_with_npm_global() {
    let command = Command::new("npm install -g typescript".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);
    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command == "sudo npm install -g typescript"));
}

#[test]
fn test_sudo_not_for_npm_local_install() {
    let command = Command::new("npm install express".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);
    let matches = corrector.find_corrections();
    assert!(matches.iter().all(|m| m.rule != "sudo_missing"));
}

// -- Docker extended tests --

#[test]
fn test_docker_typo() {
    let command = Command::new("dcoker ps".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);
    let matches = corrector.find_corrections();
    assert!(matches.iter().any(|m| m.corrected_command == "docker ps"));
}

#[test]
fn test_docker_subcommand_typo() {
    let command = Command::new("docker imags".to_string());
    let settings = Settings::default();
    let corrector = Corrector::new(command, settings);
    let matches = corrector.find_corrections();
    assert!(matches
        .iter()
        .any(|m| m.corrected_command == "docker images"));
}

#[test]
fn test_docker_subcommand_typo_variants() {
    let cases = [
        ("contianer", "container"),
        ("voluem", "volume"),
        ("ntework", "network"),
        ("psuh", "push"),
        ("pul", "pull"),
    ];
    for (typo, expected) in &cases {
        let command = Command::new(format!("docker {} arg", typo));
        let settings = Settings::default();
        let corrector = Corrector::new(command, settings);
        let matches = corrector.find_corrections();
        assert!(
            matches
                .iter()
                .any(|m| m.corrected_command == format!("docker {} arg", expected)),
            "expected {expected} for typo {typo}"
        );
    }
}

// -- Extended git tests --

#[test]
fn test_git_subcommand_typo_extended() {
    let cases = [
        ("stahs", "stash"),
        ("mrege", "merge"),
        ("dff", "diff"),
        ("swtich", "switch"),
        ("restroe", "restore"),
        ("rebaes", "rebase"),
        ("cherr-pick", "cherry-pick"),
    ];
    for (typo, expected) in &cases {
        let command = Command::new(format!("git {} arg", typo));
        let settings = Settings::default();
        let corrector = Corrector::new(command, settings);
        let matches = corrector.find_corrections();
        assert!(
            matches
                .iter()
                .any(|m| m.corrected_command == format!("git {} arg", expected)),
            "expected {expected} for typo {typo}"
        );
    }
}
