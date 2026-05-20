use crate::types::MatchResult;

pub fn make_match(rule: &'static str, corrected_command: String, similarity: f64) -> MatchResult {
    MatchResult {
        rule,
        corrected_command,
        similarity,
    }
}

pub fn replace_first(parts: &[String], replacement: &str) -> String {
    let mut result = String::with_capacity(
        replacement.len() + parts[1..].iter().map(|p| p.len() + 1).sum::<usize>(),
    );
    result.push_str(replacement);
    for part in &parts[1..] {
        result.push(' ');
        result.push_str(part);
    }
    result
}

pub fn replace_part(parts: &[String], index: usize, replacement: &str) -> String {
    let mut result = String::with_capacity(
        replacement.len() + parts.iter().map(|p| p.len()).sum::<usize>() + parts.len() - 1,
    );
    for (i, part) in parts.iter().enumerate() {
        if i != 0 {
            result.push(' ');
        }
        if i == index {
            result.push_str(replacement);
        } else {
            result.push_str(part);
        }
    }
    result
}

pub fn prepend(parts: &[String], prefix: &[&str]) -> String {
    let prefix_len: usize = prefix.iter().map(|p| p.len()).sum::<usize>() + prefix.len();
    let suffix_len: usize = parts.iter().map(|p| p.len() + 1).sum::<usize>();
    let mut result = String::with_capacity(prefix_len + suffix_len);
    for (i, p) in prefix.iter().enumerate() {
        if i != 0 {
            result.push(' ');
        }
        result.push_str(p);
    }
    for part in parts {
        result.push(' ');
        result.push_str(part);
    }
    result
}
