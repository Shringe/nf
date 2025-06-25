use clap::Args;

use crate::cli::Actionable;

use super::cmd;

/// Strips unnecessary -- between nixpkgs and to_command arguements. For example:
/// nixpkgs#hello -- to_command -> nixpkgs#hello to_command
fn strip_delimiter(args: &mut Vec<String>) {
    if let Some(i) = cmd::index_of_flag(args, "--") {
        if args.len() > i + 1 && args[i - 1].starts_with("nixpkgs#") {
            args.remove(i);
        }
    }
}

/// Finds the first arg prefixed with nixpkgs# and removes the prefix
fn unformat_nixpkg(args: &mut Vec<String>) {
    for a in args.iter_mut() {
        if let Some(stripped) = a.strip_prefix("nixpkgs#") {
            *a = stripped.to_string();
            break;
        }
    }
}

/// Replaces the first arguement (nix) with nf
fn replace_nix(args: &mut Vec<String>) {
    args[0] = "nf".to_string()
}

#[derive(Debug, Args)]
pub struct UnProcesser {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,
}

impl Actionable for UnProcesser {
    fn perform(&self, debug: bool) {
        todo!()
    }
}

impl UnProcesser {
    /// Reverses a shell expansion. For example: nix run nixpkgs#hello -> nf run hello
    pub fn unprocess(&self) -> Vec<String> {
        assert!(self.args.len() > 1, "There must be more than 2 arguements!");

        let out = match self.args[1].as_str() {
            "run" => self.run(),
            "shell" => self.shell(),
            "develop" => self.develop(),
            _ => panic!("Can't determine command!"),
        };

        out
    }

    fn get_nixpkg_index(&self) -> Option<usize> {
        for (i, a) in self.args.iter().enumerate() {
            if a.starts_with("nixpkgs#") {
                return Some(i);
            }
        }
        None
    }

    fn get_args(&self) -> (Vec<String>, Vec<String>) {
        let mut nix_args = Vec::new();
        let mut program_args = Vec::new();

        let mut to_program = false;
        for a in self.args[2..].iter() {
            if a == "--" {
                to_program = true;
                continue;
            } else if a.starts_with("nixpkgs#"){
                continue;
            }
            
            if to_program {
                program_args.push(a.to_string());
            } else {
                nix_args.push(a.to_string());
            }

        }

        (nix_args, program_args)
    }

    fn get_args_to_program(&self, delimiter_index: Option<usize>) -> Vec<String> {
        if let Some(delimiter) = delimiter_index {
            self.args[delimiter+1..].to_owned()
        } else {
            Vec::new()
        }
    }

    fn run(&self) -> Vec<String> {
        let prefix = cmd::from_string("nf run");
        let nixpkg_index = self.get_nixpkg_index();

        let args = self.get_args(); 
        let nix_args = args.0;
        let program_args = args.1;

        let mut out = prefix;
        if let Some(nixpkg) = nixpkg_index {
            out.push(self.args[nixpkg].strip_prefix("nixpkgs#").unwrap().to_owned());
        }

        let needs_delimiter = !nix_args.is_empty();
        out.extend(nix_args);
        if needs_delimiter {
            out.push("--".to_string());
        }

        out.extend(program_args);

        out
    }

    fn shell(&self) -> Vec<String> {
        todo!();
    }

    fn develop(&self) -> Vec<String> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::expansions::cmd::{self, validate_processer_test};

    use super::UnProcesser;

    fn test_unprocesser(input: Vec<String>, expected: Vec<String>) {
        let up = UnProcesser { args: input.clone() };
        let out = up.unprocess();
        validate_processer_test(&input, &expected, &out);
    }

    #[test]
    fn get_nixpkg_index() {
        let up = UnProcesser { args: cmd::from_string("nix run nixpkgs#eza") };
        assert_eq!(up.get_nixpkg_index(), Some(2));
        let up = UnProcesser { args: cmd::from_string("nix run") };
        assert_eq!(up.get_nixpkg_index(), None);
    }

    #[test]
    fn get_args_to_nix() {
        let map = HashMap::from([
            ("nix run nixpkgs#eza", ""),
            ("nix run nixpkgs#eza to_nix_after", "to_nix_after"),
            ("nix run to_nix_before nixpkgs#eza to_nix_after", "to_nix_before to_nix_after"),
            ("nix run to_nix_before nixpkgs#eza to_nix_after -- to_program", "to_nix_before to_nix_after "),
            ("nix run nixpkgs#eza -- to_program_one to_program_two", ""),
        ]);

        for (k, v) in map {
            let input = cmd::from_string(k);
            let expected = cmd::from_string(v);
            let up = UnProcesser { args: input.clone() };
            let out = up.get_args().0;
            validate_processer_test(&input, &expected, &out);
        }
    }

    #[test]
    fn get_args_to_program() {
        let map = HashMap::from([
            ("nix run nixpkgs#eza", ""),
            ("nix run nixpkgs#eza to_nix_after", ""),
            ("nix run to_nix_before nixpkgs#eza to_nix_after", ""),
            ("nix run to_nix_before nixpkgs#eza to_nix_after -- to_program", "to_program"),
            ("nix run nixpkgs#eza -- to_program_one to_program_two", "to_program_one to_program_two"),
        ]);

        for (k, v) in map {
            let input = cmd::from_string(k);
            let expected = cmd::from_string(v);
            let up = UnProcesser { args: input.clone() };
            let out = up.get_args().1;
            validate_processer_test(&input, &expected, &out);
        }
    }

    #[test]
    fn replace_nix() {
        let input    = cmd::from_string("nix run");
        let expected = cmd::from_string("nf run");
        let mut out = input.clone();
        super::replace_nix(&mut out);
        validate_processer_test(&input, &expected, &out);
    }

    #[test]
    fn strip_delimiter() {
        let input    = cmd::from_string("nix run nixpkgs#hello -- to_command");
        let expected = cmd::from_string("nix run nixpkgs#hello to_command");
        let mut out = input.clone();
        super::strip_delimiter(&mut out);
        validate_processer_test(&input, &expected, &out);
    }

    #[test]
    fn unformat_nixpkg() {
        let input    = cmd::from_string("nix run nixpkgs#hello");
        let expected = cmd::from_string("nix run hello");
        let mut out = input.clone();
        super::unformat_nixpkg(&mut out);
        validate_processer_test(&input, &expected, &out);
    }

    #[test]
    fn nix_run() {
        let input    = cmd::from_string("nix run");
        let expected = cmd::from_string("nf run");
        test_unprocesser(input, expected);
    }

    #[test]
    fn nix_run_nixpkg() {
        let input    = cmd::from_string("nix run nixpkgs#eza");
        let expected = cmd::from_string("nf run eza");
        test_unprocesser(input, expected);
    }

    #[test]
    fn nix_run_with_arg_nixpkg() {
        let input    = cmd::from_string("nix run to_nix_run nixpkgs#eza");
        let expected = cmd::from_string("nf run eza to_nix_run --");
        test_unprocesser(input, expected);
    }

    #[test]
    fn nix_run_nixpkg_with_arg() {
        let input    = cmd::from_string("nix run nixpkgs#eza -- to_command");
        let expected = cmd::from_string("nf run eza to_command");
        test_unprocesser(input, expected);
    }

    #[test]
    fn nix_run_with_arg_nixpkg_with_arg() {
        let input    = cmd::from_string("nix run to_nix_run nixpkgs#eza -- to_command");
        let expected = cmd::from_string("nf run eza to_nix_run -- to_command");
        test_unprocesser(input, expected);
    }
}
