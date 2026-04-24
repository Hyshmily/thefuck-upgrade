use crate::types::{Command, MatchResult};

pub mod cd;
pub mod docker;
pub mod git;
pub mod python;
pub mod sudo;

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
                    name: "docker_compose_v2",
                    apply: docker::docker_compose_v2_rule,
                },
                RuleDefinition {
                    name: "docker_legacy_management",
                    apply: docker::docker_legacy_management_rule,
                },
                RuleDefinition {
                    name: "sudo_missing",
                    apply: sudo::sudo_missing_rule,
                },
                RuleDefinition {
                    name: "cd_correction",
                    apply: cd::cd_correction_rule,
                },
            ],
        }
    }

    pub fn register(&mut self, rule: RuleDefinition) {
        self.rules.push(rule);
    }

    pub fn match_command(
        &self,
        command: &Command,
        enabled_rules: &[String],
        excluded_rules: &[String],
    ) -> Vec<MatchResult> {
        self.rules
            .iter()
            .filter(|rule| is_enabled(rule.name, enabled_rules, excluded_rules))
            .filter_map(|rule| (rule.apply)(command))
            .collect()
    }
}

fn is_enabled(rule_name: &str, enabled_rules: &[String], excluded_rules: &[String]) -> bool {
    if excluded_rules.iter().any(|rule| rule == rule_name) {
        return false;
    }

    enabled_rules.is_empty()
        || enabled_rules.iter().any(|rule| rule == "All rules enabled")
        || enabled_rules.iter().any(|rule| rule == rule_name)
}
