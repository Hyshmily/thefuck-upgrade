pub fn levenshtein(s1: &str, s2: &str) -> usize {
    let s1 = s1.chars().collect::<Vec<_>>();
    let s2 = s2.chars().collect::<Vec<_>>();

    let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];

    for i in 0..=s1.len() {
        dp[i][0] = i;
    }

    for j in 0..=s2.len() {
        dp[0][j] = j;
    }

    for i in 1..=s1.len() {
        for j in 1..=s2.len() {
            if s1[i - 1] == s2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = (dp[i - 1][j] + 1)
                    .min(dp[i][j - 1] + 1)
                    .min(dp[i - 1][j - 1] + 1);
            }
        }
    }

    dp[s1.len()][s2.len()]
}

pub fn levenshtein_ratio(s1: &str, s2: &str) -> f64 {
    let distance = levenshtein(s1, s2);
    let max_len = s1.len().max(s2.len());
    if max_len == 0 {
        1.0
    } else {
        1.0 - (distance as f64 / max_len as f64)
    }
}

pub const SIMILARITY_TYPO: f64 = 0.97;
pub const SIMILARITY_SUBCOMMAND_TYPO: f64 = 0.96;
pub const SIMILARITY_BRANCH: f64 = 0.95;
pub const SIMILARITY_MIGRATION: f64 = 0.94;
pub const SIMILARITY_LEGACY: f64 = 0.92;
pub const SIMILARITY_FORCE: f64 = 0.93;
pub const SIMILARITY_UPSTREAM: f64 = 0.9;
pub const SIMILARITY_SUDO: f64 = 0.88;
pub const SIMILARITY_COMPUTED: f64 = 0.0;
