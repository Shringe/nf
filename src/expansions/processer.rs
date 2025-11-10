use std::path::Path;

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

/// Determines whether there is a nested flake in the CWD
fn is_nested_flake() -> bool {
    Path::new("./flake/flake.nix").is_file()
}

pub trait Processer {
    /// Processes the shell expansion.
    fn process(&self) -> Vec<String>;

    /// Processes and executes the shell expansion.
    /// If dryrun == true, then just println!() the expansion instead.
    fn execute(&self, dryrun: bool) {
        let cmd = self.process();

        log::debug!("> {}", cmd::to_string(&cmd));
        if !dryrun {
            cmd::execute_to_stdout(&cmd);
        }
    }
}

#[derive(Debug, Args)]
pub struct Run {
    /// Arguments for the program. If you have arguments for the nix_cli, then place them before
    /// a delimiter.
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
    fn perform(&self, dryrun: bool) {
        self.execute(dryrun);
    }
}

#[derive(Debug, Args)]
pub struct Shell {
    /// Arguments for the program. If you have arguments for the nix_cli, then place them before
    /// a delimiter.
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,

    /// If this is set to config, it will pull from the config file instead
    #[arg(long, default_value = "config")]
    shell: String,
}

impl Processer for Shell {
    fn process(&self) -> Vec<String> {
        let mut out = cmd::from_string("nix shell");
        let config = ConfigFile::new();

        if !self.args.is_empty() {
            out.push(format_nixpkg(&self.args[0]));
            out.extend_from_slice(&self.args[1..]);
        } else if let Ok(c) = &config {
            if c.nested_flakes && is_nested_flake() {
                out.push("./flake".to_string());
            }
        };

        if !cmd::contains_flag(&self.args, "--command") {
            out.push("--command".to_string());

            let shell = if let Ok(c) = config {
                c.shell
            } else {
                self.shell.clone()
            };

            out.push(shell);
        }

        out
    }
}

impl Actionable for Shell {
    fn perform(&self, dryrun: bool) {
        self.execute(dryrun);
    }
}

#[derive(Debug, Args)]
pub struct Develop {
    /// Arguments for the program. If you have arguments for the nix_cli, then place them before
    /// a delimiter.
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,

    /// If this is set to config, it will pull from the config file instead
    #[arg(long, default_value = "config")]
    shell: String,
}

impl Processer for Develop {
    fn process(&self) -> Vec<String> {
        let mut out = cmd::from_string("nix develop");
        let config = ConfigFile::new();

        if !self.args.is_empty() {
            out.push(format_nixpkg(&self.args[0]));
            out.extend_from_slice(&self.args[1..]);
        } else if let Ok(c) = &config {
            if c.nested_flakes && is_nested_flake() {
                out.push("./flake".to_string());
            }
        };

        if !cmd::contains_flag(&self.args, "--command") {
            out.push("--command".to_string());

            let shell = if let Ok(c) = config {
                c.shell
            } else {
                self.shell.clone()
            };

            out.push(shell);
        }

        out
    }
}

impl Actionable for Develop {
    fn perform(&self, dryrun: bool) {
        self.execute(dryrun);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        expansions::cmd::{self, validate_processer_test},
        mode::Mode,
    };

    use super::{Develop, Processer, Run, Shell};

    const SHELL: &str = "zsh";

    fn test_processer<P: Processer>(input: Vec<String>, expected: Vec<String>, p: P) {
        validate_processer_test(&input, &expected, &p.process());
    }

    fn test_processer_any(input: Vec<String>, expected: Vec<String>, mode: &Mode) {
        match mode {
            Mode::Run(_) => {
                let p = Run {
                    args: input.clone(),
                };
                test_processer(input, expected, p);
            }
            Mode::Shell(_) => {
                let p = Shell {
                    args: input.clone(),
                    shell: SHELL.to_string(),
                };
                test_processer(input, expected, p);
            }
            Mode::Develop(_) => {
                let p = Develop {
                    args: input.clone(),
                    shell: SHELL.to_string(),
                };
                test_processer(input, expected, p);
            }
            _ => panic!("Wrong mode!"),
        };
    }

    fn test_processer_map(map: HashMap<&str, &str>, mode: Mode) {
        for (k, v) in map {
            test_processer_any(cmd::from_string(k), cmd::from_string(v), &mode);
        }
    }

    #[test]
    fn nix_run() {
        let map = HashMap::from([
            ("", "nix run"),
            ("eza", "nix run nixpkgs#eza"),
            ("eza to_nix --", "nix run nixpkgs#eza to_nix --"),
            ("eza to_program", "nix run nixpkgs#eza -- to_program"),
            ("eza -- to_program", "nix run nixpkgs#eza -- to_program"),
            (
                "eza to_nix -- to_program",
                "nix run nixpkgs#eza to_nix -- to_program",
            ),
        ]);

        test_processer_map(map, Mode::Run(Run { args: Vec::new() }));
    }

    #[test]
    fn nix_shell() {
        let map = HashMap::from([
            ("", "nix shell --command zsh"),
            ("eza", "nix shell nixpkgs#eza --command zsh"),
            ("--help", "nix shell --help --command zsh"),
            ("eza --help", "nix shell nixpkgs#eza --help --command zsh"),
            ("--command bash", "nix shell --command bash"),
        ]);

        test_processer_map(
            map,
            Mode::Shell(Shell {
                args: Vec::new(),
                shell: SHELL.to_string(),
            }),
        );
    }

    #[test]
    fn nix_develop() {
        let map = HashMap::from([
            ("", "nix develop --command zsh"),
            ("eza", "nix develop nixpkgs#eza --command zsh"),
            ("--help", "nix develop --help --command zsh"),
            ("eza --help", "nix develop nixpkgs#eza --help --command zsh"),
            ("--command bash", "nix develop --command bash"),
        ]);

        test_processer_map(
            map,
            Mode::Develop(Develop {
                args: Vec::new(),
                shell: SHELL.to_string(),
            }),
        );
    }
}
