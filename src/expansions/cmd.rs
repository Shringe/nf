use std::{os::unix::process::CommandExt, process::Command};

/// Asserts that input = expected with a pretty failure message
#[cfg(test)]
pub fn validate_processer_test(input: &Vec<String>, expected: &Vec<String>, out: &Vec<String>) {
    assert_eq!(
        expected,
        out,
        "\n    Input: {}\n Expected: {}\n   Actual: {}",
        to_string(input),
        to_string(expected),
        to_string(out)
    );
}

pub fn to_string(args: &Vec<String>) -> String {
    args.join(" ")
}

pub fn from_string<S: AsRef<str>>(args: S) -> Vec<String> {
    args.as_ref()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

/// Check if args contain a specific flag
pub fn contains_flag(args: &Vec<String>, flag: &str) -> bool {
    args.iter().any(|arg| arg == flag)
}

/// Replaces the current process with a new one.
/// Primarily used for executing shell expansions.
pub fn execute_to_stdout(args: &Vec<String>) {
    let _ = Command::new(&args[0]).args(&args[1..]).exec(); // This replaces the current process
}
