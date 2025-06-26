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

    /// Unprocesses and returns everything after the first two arguements
    fn get_args(&self) -> Vec<String> {
        let mut pkg = None;
        let mut shell_args = Vec::new();
        let mut nix_args = Vec::new();
        let mut program_args = Vec::new();

        let mut to_program = false;
        let mut is_shell = false;
        for a in self.args[2..].iter() {
            if is_shell {
                shell_args.push("--shell".to_string());
                shell_args.push(a.to_string());
                is_shell = false;
                continue;
            }

            if a == "--" {
                to_program = true;
                continue;
            } else if a == "--command" {
                is_shell = true;
                continue;
            } else if let Some(p) = a.strip_prefix("nixpkgs#") {
                pkg = Some(p.to_string());
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

        // Plus one for the potential package
        let mut out = Vec::with_capacity(shell_args.len() + nix_args.len() + program_args.len() + 1);
        out.extend(shell_args);

        if let Some(p) = pkg {
            out.push(p);
        }

        out.extend(nix_args);
        out.extend(program_args);
        out
    }

    fn run(&self) -> Vec<String> {
        let mut out = cmd::from_string("nf run");
        out.extend(self.get_args());
        out
    }

    fn shell(&self) -> Vec<String> {
        let mut out = cmd::from_string("nf shell");
        out.extend(self.get_args());
        out
    }

    fn develop(&self) -> Vec<String> {
        let mut out = cmd::from_string("nf develop");
        out.extend(self.get_args());
        out
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

    fn test_unprocesser_map(map: HashMap<&str, &str>) {
        for (k, v) in map {
            test_unprocesser(cmd::from_string(k), cmd::from_string(v));
        }
    }

    #[test]
    fn get_args() {
        let map = HashMap::from([
            ("nix run nixpkgs#eza", "eza"),
            ("nix run nixpkgs#eza to_nix_after", "eza to_nix_after --"),
            ("nix run to_nix_before nixpkgs#eza to_nix_after", "eza to_nix_before to_nix_after --"),
            ("nix run to_nix_before nixpkgs#eza to_nix_after -- to_program", "eza to_nix_before to_nix_after -- to_program"),
            ("nix run nixpkgs#eza -- to_program_one to_program_two", "eza to_program_one to_program_two"),
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
        let map = HashMap::from([
            ("nix run", "nf run"),
            ("nix run nixpkgs#eza", "nf run eza"),
            ("nix run to_nix nixpkgs#eza", "nf run eza to_nix --"),
            ("nix run nixpkgs#eza -- to_program", "nf run eza to_program"),
            ("nix run to_nix nixpkgs#eza -- to_program", "nf run eza to_nix -- to_program"),
        ]);

        test_unprocesser_map(map);
    }

    #[test]
    fn nix_shell() {
        let map = HashMap::from([
            ("nix shell", "nf shell"),
            ("nix shell nixpkgs#hello", "nf shell hello"),
            ("nix shell to_nix nixpkgs#eza", "nf shell eza to_nix --"),
            ("nix shell nixpkgs#eza to_nix", "nf shell eza to_nix --"),
            ("nix shell nixpkgs#eza -- to_program", "nf shell eza to_program"),
            ("nix shell to_nix_one nixpkgs#eza to_nix_two", "nf shell eza to_nix_one to_nix_two --"),
            ("nix shell to_nix_one nixpkgs#eza to_nix_two -- to_program", "nf shell eza to_nix_one to_nix_two -- to_program"),
            ("nix shell nixpkgs#eza --command fish -- to_program", "nf shell --shell fish eza to_program"),
            ("nix shell --command fish nixpkgs#eza", "nf shell --shell fish eza"),
        ]);

        test_unprocesser_map(map);
    }

    #[test]
    fn nix_develop() {
        let map = HashMap::from([
            ("nix develop", "nf develop"),
            ("nix develop nixpkgs#hello", "nf develop hello"),
            ("nix develop to_nix nixpkgs#eza", "nf develop eza to_nix --"),
            ("nix develop nixpkgs#eza to_nix", "nf develop eza to_nix --"),
            ("nix develop nixpkgs#eza -- to_program", "nf develop eza to_program"),
            ("nix develop to_nix_one nixpkgs#eza to_nix_two", "nf develop eza to_nix_one to_nix_two --"),
            ("nix develop to_nix_one nixpkgs#eza to_nix_two -- to_program", "nf develop eza to_nix_one to_nix_two -- to_program"),
            ("nix develop nixpkgs#eza --command fish -- to_program", "nf develop --shell fish eza to_program"),
            ("nix develop --command fish nixpkgs#eza", "nf develop --shell fish eza"),
        ]);

        test_unprocesser_map(map);
    }
}
