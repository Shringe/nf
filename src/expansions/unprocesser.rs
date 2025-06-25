use clap::Args;

use crate::cli::Actionable;

use super::cmd;

#[derive(Debug, Args)]
pub struct UnProcesser {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,
}

impl Actionable for UnProcesser {
    fn perform(&self, debug: bool) {
        if debug {} // Gets rid of the warning since debug is not used here
        let expanded = self.unprocess();
        println!("> {}", cmd::to_string(&expanded));
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

    fn get_args(&self) -> Vec<String> {
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

        if !nix_args.is_empty() {
            nix_args.push("--".to_string());
        }

        let mut out = Vec::with_capacity(nix_args.len() + program_args.len());
        out.extend(nix_args);
        out.extend(program_args);
        out
    }

    fn get_pkg(&self) -> Option<String> {
        for a in self.args.iter() {
            if let Some(stripped) = a.strip_prefix("nixpkgs#") {
                return Some(stripped.to_string());
            }
        }

        None
    }

    fn run(&self) -> Vec<String> {
        let mut out = cmd::from_string("nf run");

        if let Some(pkg) = self.get_pkg() {
            out.push(pkg);
        }

        out.extend(self.get_args());
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
    fn get_args() {
        let map = HashMap::from([
            ("nix run nixpkgs#eza", ""),
            ("nix run nixpkgs#eza to_nix_after", "to_nix_after --"),
            ("nix run to_nix_before nixpkgs#eza to_nix_after", "to_nix_before to_nix_after --"),
            ("nix run to_nix_before nixpkgs#eza to_nix_after -- to_program", "to_nix_before to_nix_after -- to_program"),
            ("nix run nixpkgs#eza -- to_program_one to_program_two", "to_program_one to_program_two"),
        ]);

        for (k, v) in map {
            let input = cmd::from_string(k);
            let expected = cmd::from_string(v);
            let up = UnProcesser { args: input.clone() };
            let out = up.get_args();
            validate_processer_test(&input, &expected, &out);
        }
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
