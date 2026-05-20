use crate::types::{Command, MatchResult};

pub mod apt;
pub mod aws;
pub mod az;
pub mod brew;
pub mod bun;
pub mod cargo;
pub mod cd;
pub mod choco;
pub mod common;
pub mod conda;
pub mod dnf;
pub mod docker;
pub mod git;
pub mod go;
pub mod gradle;
pub mod grep;
pub mod kubectl;
pub mod maven;
pub mod missing_space;
pub mod npm;
pub mod pacman;
pub mod pip;
pub mod pnpm;
pub mod python;
pub mod sudo;
pub mod systemctl;
pub mod terraform;
pub mod uv;
pub mod winget;
pub mod yarn;

#[derive(Clone, Copy)]
pub struct RuleDefinition {
    pub name: &'static str,
    pub apply: fn(&Command) -> Option<MatchResult>,
}

pub struct RuleRegistry {
    rules: Vec<RuleDefinition>,
}

impl Default for RuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl RuleRegistry {
    pub fn new() -> Self {
        Self {
            rules: vec![
                // -- Git rules --
                RuleDefinition {
                    name: "git_command",
                    apply: git::git_typo_rule,
                },
                RuleDefinition {
                    name: "git_subcommand_typo",
                    apply: git::git_subcommand_typo_rule,
                },
                RuleDefinition {
                    name: "git_push_upstream",
                    apply: git::git_push_upstream_rule,
                },
                RuleDefinition {
                    name: "git_checkout_to_switch",
                    apply: git::git_checkout_to_switch_rule,
                },
                RuleDefinition {
                    name: "git_force_with_lease",
                    apply: git::git_force_with_lease_rule,
                },
                // -- Python rules --
                RuleDefinition {
                    name: "python_command",
                    apply: python::python_typo_rule,
                },
                RuleDefinition {
                    name: "python_pip_to_uv",
                    apply: python::pip_to_uv_rule,
                },
                RuleDefinition {
                    name: "python_pip_module",
                    apply: python::pip_to_python_module_rule,
                },
                RuleDefinition {
                    name: "pip_subcommand_typo",
                    apply: python::pip_subcommand_typo_rule,
                },
                // -- Docker rules --
                RuleDefinition {
                    name: "docker_command",
                    apply: docker::docker_typo_rule,
                },
                RuleDefinition {
                    name: "docker_compose_v2",
                    apply: docker::docker_compose_v2_rule,
                },
                RuleDefinition {
                    name: "docker_legacy_management",
                    apply: docker::docker_legacy_management_rule,
                },
                RuleDefinition {
                    name: "docker_subcommand_typo",
                    apply: docker::docker_subcommand_typo_rule,
                },
                // -- Maven rules --
                RuleDefinition {
                    name: "mvn_command",
                    apply: maven::mvn_typo_rule,
                },
                RuleDefinition {
                    name: "mvn_subcommand_typo",
                    apply: maven::mvn_subcommand_typo_rule,
                },
                RuleDefinition {
                    name: "mvn_multiphase_typo",
                    apply: maven::mvn_multiphase_typo_rule,
                },
                // -- Sudo rule --
                RuleDefinition {
                    name: "sudo_missing",
                    apply: sudo::sudo_missing_rule,
                },
                // -- cd rule --
                RuleDefinition {
                    name: "cd_correction",
                    apply: cd::cd_correction_rule,
                },
                // -- Common command typos --
                RuleDefinition {
                    name: "common_typo",
                    apply: common::common_typo_rule,
                },
                // -- npm/yarn/pnpm rules --
                RuleDefinition {
                    name: "npm_command",
                    apply: npm::npm_typo_rule,
                },
                RuleDefinition {
                    name: "npm_subcommand_typo",
                    apply: npm::npm_subcommand_typo_rule,
                },
                // -- Cargo rules --
                RuleDefinition {
                    name: "cargo_command",
                    apply: cargo::cargo_typo_rule,
                },
                RuleDefinition {
                    name: "cargo_subcommand_typo",
                    apply: cargo::cargo_subcommand_typo_rule,
                },
                // -- Go rules --
                RuleDefinition {
                    name: "go_command",
                    apply: go::go_typo_rule,
                },
                RuleDefinition {
                    name: "go_subcommand_typo",
                    apply: go::go_subcommand_typo_rule,
                },
                // -- pip command rule --
                RuleDefinition {
                    name: "pip_command",
                    apply: pip::pip_typo_rule,
                },
                // -- Brew rules --
                RuleDefinition {
                    name: "brew_command",
                    apply: brew::brew_typo_rule,
                },
                RuleDefinition {
                    name: "brew_subcommand_typo",
                    apply: brew::brew_subcommand_typo_rule,
                },
                // -- apt rules --
                RuleDefinition {
                    name: "apt_command",
                    apply: apt::apt_typo_rule,
                },
                RuleDefinition {
                    name: "apt_subcommand_typo",
                    apply: apt::apt_subcommand_typo_rule,
                },
                RuleDefinition {
                    name: "apt_get_to_apt",
                    apply: apt::apt_get_to_apt_rule,
                },
                // -- systemctl rules --
                RuleDefinition {
                    name: "systemctl_command",
                    apply: systemctl::systemctl_typo_rule,
                },
                RuleDefinition {
                    name: "systemctl_subcommand_typo",
                    apply: systemctl::systemctl_subcommand_typo_rule,
                },
                // -- kubectl rules --
                RuleDefinition {
                    name: "kubectl_command",
                    apply: kubectl::kubectl_typo_rule,
                },
                RuleDefinition {
                    name: "kubectl_subcommand_typo",
                    apply: kubectl::kubectl_subcommand_typo_rule,
                },
                // -- Terraform rules --
                RuleDefinition {
                    name: "terraform_command",
                    apply: terraform::terraform_typo_rule,
                },
                RuleDefinition {
                    name: "terraform_subcommand_typo",
                    apply: terraform::terraform_subcommand_typo_rule,
                },
                // -- Conda rules --
                RuleDefinition {
                    name: "conda_command",
                    apply: conda::conda_typo_rule,
                },
                RuleDefinition {
                    name: "conda_subcommand_typo",
                    apply: conda::conda_subcommand_typo_rule,
                },
                // -- Gradle rules --
                RuleDefinition {
                    name: "gradle_command",
                    apply: gradle::gradle_typo_rule,
                },
                RuleDefinition {
                    name: "gradle_subcommand_typo",
                    apply: gradle::gradle_subcommand_typo_rule,
                },
                // -- Bun rules --
                RuleDefinition {
                    name: "bun_command",
                    apply: bun::bun_typo_rule,
                },
                RuleDefinition {
                    name: "bun_subcommand_typo",
                    apply: bun::bun_subcommand_typo_rule,
                },
                // -- uv rules --
                RuleDefinition {
                    name: "uv_command",
                    apply: uv::uv_typo_rule,
                },
                RuleDefinition {
                    name: "uv_subcommand_typo",
                    apply: uv::uv_subcommand_typo_rule,
                },
                // -- PNPM rules --
                RuleDefinition {
                    name: "pnpm_command",
                    apply: pnpm::pnpm_typo_rule,
                },
                RuleDefinition {
                    name: "pnpm_subcommand_typo",
                    apply: pnpm::pnpm_subcommand_typo_rule,
                },
                // -- Yarn rules --
                RuleDefinition {
                    name: "yarn_command",
                    apply: yarn::yarn_typo_rule,
                },
                RuleDefinition {
                    name: "yarn_subcommand_typo",
                    apply: yarn::yarn_subcommand_typo_rule,
                },
                // -- DNF rules --
                RuleDefinition {
                    name: "dnf_command",
                    apply: dnf::dnf_typo_rule,
                },
                RuleDefinition {
                    name: "dnf_subcommand_typo",
                    apply: dnf::dnf_subcommand_typo_rule,
                },
                // -- Pacman rules --
                RuleDefinition {
                    name: "pacman_command",
                    apply: pacman::pacman_typo_rule,
                },
                RuleDefinition {
                    name: "pacman_subcommand_typo",
                    apply: pacman::pacman_subcommand_typo_rule,
                },
                // -- Choco rules --
                RuleDefinition {
                    name: "choco_command",
                    apply: choco::choco_typo_rule,
                },
                RuleDefinition {
                    name: "choco_subcommand_typo",
                    apply: choco::choco_subcommand_typo_rule,
                },
                // -- WinGet rules --
                RuleDefinition {
                    name: "winget_command",
                    apply: winget::winget_typo_rule,
                },
                RuleDefinition {
                    name: "winget_subcommand_typo",
                    apply: winget::winget_subcommand_typo_rule,
                },
                // -- AWS CLI rules --
                RuleDefinition {
                    name: "aws_command",
                    apply: aws::aws_typo_rule,
                },
                RuleDefinition {
                    name: "aws_service_typo",
                    apply: aws::aws_service_typo_rule,
                },
                // -- Azure CLI rules --
                RuleDefinition {
                    name: "az_command",
                    apply: az::az_typo_rule,
                },
                RuleDefinition {
                    name: "az_subcommand_typo",
                    apply: az::az_subcommand_typo_rule,
                },
                // -- Grep rules --
                RuleDefinition {
                    name: "grep_recursive",
                    apply: grep::grep_recursive_rule,
                },
                // -- Missing space / wrong hyphen rules --
                RuleDefinition {
                    name: "missing_space",
                    apply: missing_space::missing_space_rule,
                },
                RuleDefinition {
                    name: "wrong_hyphen",
                    apply: missing_space::wrong_hyphen_rule,
                },
            ],
        }
    }

    pub fn match_command(
        &self,
        command: &Command,
        enabled_rules: &[String],
        excluded_rules: &[String],
    ) -> Vec<MatchResult> {
        let all_on =
            enabled_rules.is_empty() || enabled_rules.iter().any(|r| r == "All rules enabled");

        self.rules
            .iter()
            .filter(|rule| {
                if excluded_rules.iter().any(|r| r == rule.name) {
                    return false;
                }
                all_on || enabled_rules.iter().any(|r| r == rule.name)
            })
            .filter_map(|rule| (rule.apply)(command))
            .collect()
    }
}
