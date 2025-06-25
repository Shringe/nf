use std::{os::unix::process::CommandExt, process::Command};

/// Asserts that input = expected with a pretty failure message
pub fn validate_processer_test(input: &Vec<String>, expected: &Vec<String>, out: &Vec<String>) {
    assert_eq!(
        out,
        expected,
        "\n    Input: {}\n Expected: {}\n   Actual: {}",
        to_string(input), to_string(expected), to_string(out)
    );
}

pub fn to_string(args: &Vec<String>) -> String {
    args.join(" ")
}

pub fn from_string<S: AsRef<str>>(args: S) -> Vec<String> {
    args.as_ref().split_whitespace().map(|s| s.to_string()).collect()
}

pub fn from_vec<S: Into<String>>(args: Vec<S>) -> Vec<String> {
    args.into_iter().map(|s| s.into()).collect()
}

/// Check if args contain a specific flag
pub fn contains_flag(args: &Vec<String>, flag: &str) -> bool {
    args.iter().any(|arg| arg == flag)
}

/// Finds the index of a flag if it exists
pub fn index_of_flag(args: &Vec<String>, flag: &str) -> Option<usize> {
    for (i, arg) in args.iter().enumerate() {
        if arg == flag {
            return Some(i);
        }
    }

    None
}

/// Replaces the current process with a new one.
/// Primarily used for executing shell expansions.
pub fn execute_to_stdout(args: &Vec<String>) {
    let _ = Command::new(&args[0])
        .args(&args[1..])
        .exec(); // This replaces the current process
}
