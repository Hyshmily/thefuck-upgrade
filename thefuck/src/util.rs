pub fn levenshtein(s1: &str, s2: &str) -> usize {
    let (s1, s2): (Vec<char>, Vec<char>) = if s1.len() < s2.len() {
        (s1.chars().collect(), s2.chars().collect())
    } else {
        (s2.chars().collect(), s1.chars().collect())
    };

    let mut prev: Vec<usize> = (0..=s2.len()).collect();
    let mut curr = vec![0; s2.len() + 1];

    for (i, c1) in s1.iter().enumerate() {
        curr[0] = i + 1;
        for (j, c2) in s2.iter().enumerate() {
            curr[j + 1] = if c1 == c2 {
                prev[j]
            } else {
                1 + prev[j].min(prev[j + 1]).min(curr[j])
            };
        }
        std::mem::swap(&mut prev, &mut curr);
    }

    prev[s2.len()]
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
