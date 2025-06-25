use clap::Args;

use crate::{cli::Actionable, config::manager::ConfigFile};

use super::cmd;

/// pkg -> nixpkgs#pkg
/// Avoids treating args as pkgs
fn format_nixpkg(pkg: &str) -> String {
    if pkg.starts_with('-') {
        pkg.to_string()
    } else {
        format!("nixpkgs#{}", pkg)
    }
}

pub trait Processer {
    /// Processes the shell expansion.
    fn process(&self) -> Vec<String>;

    /// Processes and executes the shell expansion.
    /// If debug == true, then just println!() the expansion instead.
    fn execute(&self, debug: bool) {
        let cmd = self.process();

        if debug {
            println!("> {}", cmd.join(" "));
        } else {
            cmd::execute_to_stdout(&cmd);
        }
    }
}

#[derive(Debug, Args)]
pub struct Run {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,
}

impl Processer for Run {
    fn process(&self) -> Vec<String> {
        let prefix = cmd::from_string("nix run");
        if self.args.is_empty() {
            return prefix;
        }

        let mut out = Vec::new();
        out.extend(prefix);
        
        out.push(format_nixpkg(&self.args[0]));
        
        if self.args.len() > 1 {
            if !cmd::contains_flag(&self.args, "--") {
                out.push("--".to_string());
            }

            out.extend_from_slice(&self.args[1..]);
        }
        
        out
    }
}

impl Actionable for Run {
    fn perform(&self, debug: bool) {
        self.execute(debug);
    }
}

#[derive(Debug, Args)]
pub struct Shell {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,

    /// If this is set to config, it will pull from the config file
    #[arg(long, default_value = "config")]
    shell: String,
}

impl Processer for Shell {
    fn process(&self) -> Vec<String> {
        let mut out = cmd::from_string("nix shell");
        
        if !self.args.is_empty() {
            out.push(format_nixpkg(&self.args[0]));
            out.extend_from_slice(&self.args[1..]);
        }
        
        if !cmd::contains_flag(&self.args, "--command") {
            out.push("--command".to_string());

            let shell = if self.shell == "config" {
                ConfigFile::default().shell
            } else {
                self.shell.clone()
            };

            out.push(shell);
        }
        
        out
    }
}

impl Actionable for Shell {
    fn perform(&self, debug: bool) {
        self.execute(debug);
    }
}

#[derive(Debug, Args)]
pub struct Develop {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,

    /// If this is set to config, it will pull from the config file
    #[arg(long, default_value = "config")]
    shell: String,
}

impl Processer for Develop {
    fn process(&self) -> Vec<String> {
        let mut out = cmd::from_string("nix develop");
        
        if !self.args.is_empty() {
            out.push(format_nixpkg(&self.args[0]));
            out.extend_from_slice(&self.args[1..]);
        }
        
        if !cmd::contains_flag(&self.args, "--command") {
            out.push("--command".to_string());

            let shell = if self.shell == "config" {
                ConfigFile::default().shell
            } else {
                self.shell.clone()
            };

            out.push(shell);
        }
        
        out
    }
}

impl Actionable for Develop {
    fn perform(&self, debug: bool) {
        self.execute(debug);
    }
}

#[cfg(test)]
mod tests {
    use crate::expansions::cmd::{self, validate_processer_test};

    use super::{Develop, Processer, Run, Shell};

    const SHELL: &str = "zsh";

    fn test_run(input: Vec<String>, expected: Vec<String>) {
        let p = Run { args: input.clone() };
        let out = p.process();
        validate_processer_test(&input, &expected, &out);
    }

    fn test_shell(input: Vec<String>, expected: Vec<String>) {
        let p = Shell { args: input.clone(), shell: SHELL.to_string() };
        let out = p.process();
        validate_processer_test(&input, &expected, &out);
    }

    fn test_develop(input: Vec<String>, expected: Vec<String>) {
        let p = Develop { args: input.clone(), shell: SHELL.to_string() };
        let out = p.process();
        validate_processer_test(&input, &expected, &out);
    }
    
    #[test]
    fn nix_run() {
        let input    = cmd::from_string("");
        let expected = cmd::from_string("nix run");
        test_run(input, expected);
    }
    
    #[test]
    fn nix_run_nixpkg() {
        let input    = cmd::from_string("eza");
        let expected = cmd::from_string("nix run nixpkgs#eza");
        test_run(input, expected);
    }
    
    #[test]
    fn nix_run_with_arg_nixpkg() {
        let input    = cmd::from_string("eza to_nix_run --");
        let expected = cmd::from_string("nix run nixpkgs#eza to_nix_run --");
        test_run(input, expected);
    }
    
    #[test]
    fn nix_run_nixpkg_with_arg() {
        let input    = cmd::from_string("eza to_command");
        let expected = cmd::from_string("nix run nixpkgs#eza -- to_command");
        test_run(input, expected);
    }
    
    #[test]
    fn nix_run_nixpkg_with_arg_redundant() {
        let input    = cmd::from_string("eza -- to_command");
        let expected = cmd::from_string("nix run nixpkgs#eza -- to_command");
        test_run(input, expected);
    }
    
    #[test]
    fn nix_run_with_arg_nixpkg_with_arg() {
        let input    = cmd::from_string("eza to_nix_run -- to_command");
        // TODO, this is in the wrong order. It probably should be this:
        // let expected = cmd::from_string("nix run to_nix_run nixpkgs#eza -- to_command");
        let expected = cmd::from_string("nix run nixpkgs#eza to_nix_run -- to_command");
        test_run(input, expected);
    }
    
    #[test]
    fn nix_shell() {
        let input    = cmd::from_string("");
        let expected = cmd::from_string("nix shell --command zsh");
        test_shell(input, expected);
    }
    
    #[test]
    fn nix_shell_nixpkg() {
        let input    = cmd::from_string("eza");
        let expected = cmd::from_string("nix shell nixpkgs#eza --command zsh");
        test_shell(input, expected);
    }
    
    #[test]
    fn nix_shell_with_arg() {
        let input    = cmd::from_string("--help");
        let expected = cmd::from_string("nix shell --help --command zsh");
        test_shell(input, expected);
    }
    
    #[test]
    fn nix_shell_with_arg_nixpkg() {
        let input    = cmd::from_string("eza --help");
        let expected = cmd::from_string("nix shell nixpkgs#eza --help --command zsh");
        test_shell(input, expected);
    }
    
    #[test]
    fn nix_shell_with_shell_specified() {
        let input    = cmd::from_string("--command bash");
        let expected = cmd::from_string("nix shell --command bash");
        test_shell(input, expected);
    }
    
    #[test]
    fn nix_develop() {
        let input    = cmd::from_string("");
        let expected = cmd::from_string("nix develop --command zsh");
        test_develop(input, expected);
    }
    
    #[test]
    fn nix_develop_nixpkg() {
        let input    = cmd::from_string("eza");
        let expected = cmd::from_string("nix develop nixpkgs#eza --command zsh");
        test_develop(input, expected);
    }
    
    #[test]
    fn nix_develop_with_arg() {
        let input    = cmd::from_string("--help");
        let expected = cmd::from_string("nix develop --help --command zsh");
        test_develop(input, expected);
    }
    
    #[test]
    fn nix_develop_with_arg_nixpkg() {
        let input    = cmd::from_string("eza --help");
        let expected = cmd::from_string("nix develop nixpkgs#eza --help --command zsh");
        test_develop(input, expected);
    }
    
    #[test]
    fn nix_develop_with_shell_specified() {
        let input    = cmd::from_string("--command bash");
        let expected = cmd::from_string("nix develop --command bash");
        test_develop(input, expected);
    }
}
