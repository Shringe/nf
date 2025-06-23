use clap::Args;
use std::{os::unix::process::CommandExt, process::Command};

use crate::{cli::Actionable, config::manager::ConfigManager};

/// Replaces the current process with a new one.
/// Primarily used for executing shell expansions.
fn execute_to_stdout(cmd: &[String]) {
    let _ = Command::new(&cmd[0])
        .args(&cmd[1..])
        .exec(); // This replaces the current process
}

/// pkg -> nixpkgs#pkg
/// Avoids treating args as pkgs
fn format_nixpkg(pkg: &str) -> String {
    if pkg.starts_with('-') {
        pkg.to_string()
    } else {
        format!("nixpkgs#{}", pkg)
    }
}

/// Check if args contain a specific flag
fn contains_flag(args: &Vec<String>, flag: &str) -> bool {
    args.iter().any(|arg| arg == flag)
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
            execute_to_stdout(&cmd);
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
        if self.args.is_empty() {
            return vec!["nix".to_string(), "run".to_string()];
        }

        let mut out = Vec::with_capacity(self.args.len() + 4); // Pre-allocate
        out.push("nix".to_string());
        out.push("run".to_string());
        
        out.push(format_nixpkg(&self.args[0]));
        
        if self.args.len() > 1 {
            if !contains_flag(&self.args, "--") {
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
        let mut out = Vec::with_capacity(self.args.len() + 4); // Pre-allocate
        out.push("nix".to_string());
        out.push("shell".to_string());
        
        if !self.args.is_empty() {
            out.push(format_nixpkg(&self.args[0]));
            out.extend_from_slice(&self.args[1..]);
        }
        
        if !contains_flag(&self.args, "--command") {
            out.push("--command".to_string());

            let shell = if self.shell == "config" {
                let config = ConfigManager::new(false);
                config.config_file.shell
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
        let mut out = Vec::with_capacity(self.args.len() + 4); // Pre-allocate
        out.push("nix".to_string());
        out.push("develop".to_string());
        
        if !self.args.is_empty() {
            out.push(format_nixpkg(&self.args[0]));
            out.extend_from_slice(&self.args[1..]);
        }
        
        if !contains_flag(&self.args, "--command") {
            out.push("--command".to_string());

            let shell = if self.shell == "config" {
                let config = ConfigManager::new(false);
                config.config_file.shell
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
    use super::{Develop, Processer, Run, Shell};

    const SHELL: &str = "zsh";

    /// Converts vec![ "a" "b" "c" ] to vec![ "a".to_string() "b".to_string() "c".to_string() ]
    macro_rules! args {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }
    
    #[test]
    fn nix_run() {
        let args = args![];
        let p = Run { args };
        assert_eq!(p.process(), args!["nix", "run"]);
    }
    
    #[test]
    fn nix_run_nixpkg() {
        let args = args!["eza"];
        let p = Run { args };
        assert_eq!(p.process(), args!["nix", "run", "nixpkgs#eza"]);
    }
    
    #[test]
    fn nix_run_with_arg_nixpkg() {
        let args = args!["eza", "to_nix_run", "--"];
        let p = Run { args };
        assert_eq!(p.process(), args!["nix", "run", "nixpkgs#eza", "to_nix_run", "--"]);
    }
    
    #[test]
    fn nix_run_nixpkg_with_arg() {
        let args = args!["eza", "to_command"];
        let p = Run { args };
        assert_eq!(p.process(), args!["nix", "run", "nixpkgs#eza", "--", "to_command"]);
    }
    
    #[test]
    fn nix_run_nixpkg_with_arg_redundant() {
        let args = args!["eza", "--", "to_command"];
        let p = Run { args };
        assert_eq!(p.process(), args!["nix", "run", "nixpkgs#eza", "--", "to_command"]);
    }
    
    #[test]
    fn nix_run_with_arg_nixpkg_with_arg() {
        let args = args!["eza", "to_nix_run", "--", "to_command"];
        let p = Run { args };
        assert_eq!(p.process(), args!["nix", "run", "nixpkgs#eza", "to_nix_run", "--", "to_command"]);
    }
    
    #[test]
    fn nix_shell() {
        let args = args![];
        let p = Shell { args, shell: SHELL.to_string() };
        assert_eq!(p.process(), args!["nix", "shell", "--command", SHELL]);
    }
    
    #[test]
    fn nix_shell_nixpkg() {
        let args = args!["eza"];
        let p = Shell { args, shell: SHELL.to_string() };
        assert_eq!(p.process(), args!["nix", "shell", "nixpkgs#eza", "--command", SHELL]);
    }
    
    #[test]
    fn nix_shell_with_arg() {
        let args = args!["--help"];
        let p = Shell { args, shell: SHELL.to_string() };
        assert_eq!(p.process(), args!["nix", "shell", "--help", "--command", SHELL]);
    }
    
    #[test]
    fn nix_shell_with_arg_nixpkg() {
        let args = args!["eza", "--help"];
        let p = Shell { args, shell: SHELL.to_string() };
        assert_eq!(p.process(), args!["nix", "shell", "nixpkgs#eza", "--help", "--command", SHELL]);
    }
    
    #[test]
    fn nix_shell_with_shell_specified() {
        let args = args!["--command", "bash"];
        let p = Shell { args, shell: SHELL.to_string() };
        assert_eq!(p.process(), args!["nix", "shell", "--command", "bash"]);
    }
    
    #[test]
    fn nix_develop() {
        let args = args![];
        let p = Develop { args, shell: SHELL.to_string() };
        assert_eq!(p.process(), args!["nix", "develop", "--command", SHELL]);
    }
    
    #[test]
    fn nix_develop_nixpkg() {
        let args = args!["eza"];
        let p = Develop { args, shell: SHELL.to_string() };
        assert_eq!(p.process(), args!["nix", "develop", "nixpkgs#eza", "--command", SHELL]);
    }
    
    #[test]
    fn nix_develop_with_arg() {
        let args = args!["--help"];
        let p = Develop { args, shell: SHELL.to_string() };
        assert_eq!(p.process(), args!["nix", "develop", "--help", "--command", SHELL]);
    }
    
    #[test]
    fn nix_develop_with_arg_nixpkg() {
        let args = args!["eza", "--help"];
        let p = Develop { args, shell: SHELL.to_string() };
        assert_eq!(p.process(), args!["nix", "develop", "nixpkgs#eza", "--help", "--command", SHELL]);
    }
    
    #[test]
    fn nix_develop_with_shell_specified() {
        let args = args!["--command", "bash"];
        let p = Develop { args, shell: SHELL.to_string() };
        assert_eq!(p.process(), args!["nix", "develop", "--command", "bash"]);
    }
}
