/// Converts vec![ "a" "b" "c" ] to vec![ "a".to_string() "b".to_string() "c".to_string() ]
macro_rules! args {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

pub struct Processer {
    args: Vec<String>,
}

impl Processer {
    pub fn new(args: Vec<String>) -> Self {
        Self {
            args
        }
    }

    pub fn nix_run(&self) -> String {
        match self.args.len() {
            0 => return "nix run".to_string(),
            1 => return format!("nix run nixpkgs#{}", self.args[0]),
            _ => {},
        }

        let mut args = self.args.clone();
        let mut out = args![ "nix", "run" ];
        out.push(format!("nixpkgs#{}", args.remove(0)));

        if !self.args.contains(&"--".to_string()) {
            out.push("--".to_string());
        }

        for a in args {
            out.push(a);
        }

        out.join(" ")
    }

    pub fn nix_shell(&self) -> String {
        let mut args = self.args.clone();
        let mut out = args![ "nix", "shell" ];

        let len = self.args.len();

        if len > 0 {
            let pkg = args.remove(0);

            // To avoid nixpkgs#--arguement
            if !pkg.starts_with("-") {
                out.push(format!("nixpkgs#{}", pkg));
            } else {
                out.push(pkg);
            }

            for a in args {
                out.push(a);
            }
        }

        if !self.args.contains(&"--command".to_string()) {
            out.extend(args![ "--command", "fish" ]);
        }

        out.join(" ")
    }

    pub fn nix_develop(&self) -> String {
        let mut args = self.args.clone();
        let mut out = args![ "nix", "develop" ];

        let len = self.args.len();

        if len > 0 {
            let pkg = args.remove(0);

            // To avoid nixpkgs#--arguement
            if !pkg.starts_with("-") {
                out.push(format!("nixpkgs#{}", pkg));
            } else {
                out.push(pkg);
            }

            for a in args {
                out.push(a);
            }
        }

        if !self.args.contains(&"--command".to_string()) {
            out.extend(args![ "--command", "fish" ]);
        }

        out.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::Processer;

    #[test]
    fn nix_run() {
        let args = args![];
        let p = Processer::new(args);
        assert_eq!(p.nix_run(), "nix run");
    }

    #[test]
    fn nix_run_nixpkg() {
        let args = args![ "eza" ];
        let p = Processer::new(args);

        assert_eq!(p.nix_run(), "nix run nixpkgs#eza");
    }

    #[test]
    fn nix_run_with_arg_nixpkg() {
        let args = args![ "eza", "to_nix_run", "--" ];
        let p = Processer::new(args);

        assert_eq!(p.nix_run(), "nix run nixpkgs#eza to_nix_run --");
    }

    #[test]
    fn nix_run_nixpkg_with_arg() {
        let args = args![ "eza", "to_command" ];
        let p = Processer::new(args);

        assert_eq!(p.nix_run(), "nix run nixpkgs#eza -- to_command");
    }

    #[test]
    fn nix_run_nixpkg_with_arg_redundant() {
        let args = args![ "eza", "--", "to_command" ];
        let p = Processer::new(args);

        assert_eq!(p.nix_run(), "nix run nixpkgs#eza -- to_command");
    }

    #[test]
    fn nix_run_with_arg_nixpkg_with_arg() {
        let args = args![ "eza", "to_nix_run", "--", "to_command" ];
        let p = Processer::new(args);

        assert_eq!(p.nix_run(), "nix run nixpkgs#eza to_nix_run -- to_command");
    }

    #[test]
    fn nix_shell() {
        let args = args![];
        let p = Processer::new(args);

        assert_eq!(p.nix_shell(), "nix shell --command fish");
    }

    #[test]
    fn nix_shell_nixpkg() {
        let args = args![ "eza" ];
        let p = Processer::new(args);

        assert_eq!(p.nix_shell(), "nix shell nixpkgs#eza --command fish");
    }

    #[test]
    fn nix_shell_with_arg() {
        let args = args![ "--help" ];
        let p = Processer::new(args);

        assert_eq!(p.nix_shell(), "nix shell --help --command fish");
    }

    #[test]
    fn nix_shell_with_arg_nixpkg() {
        let args = args![ "eza", "--help" ];
        let p = Processer::new(args);

        assert_eq!(p.nix_shell(), "nix shell nixpkgs#eza --help --command fish");
    }

    #[test]
    fn nix_shell_with_shell_specified() {
        let args = args![ "--command", "zsh" ];
        let p = Processer::new(args);

        assert_eq!(p.nix_shell(), "nix shell --command zsh");
    }

    #[test]
    fn nix_develop() {
        let args = args![];
        let p = Processer::new(args);

        assert_eq!(p.nix_develop(), "nix develop --command fish");
    }

    #[test]
    fn nix_develop_nixpkg() {
        let args = args![ "eza" ];
        let p = Processer::new(args);

        assert_eq!(p.nix_develop(), "nix develop nixpkgs#eza --command fish");
    }

    #[test]
    fn nix_develop_with_arg() {
        let args = args![ "--help" ];
        let p = Processer::new(args);

        assert_eq!(p.nix_develop(), "nix develop --help --command fish");
    }

    #[test]
    fn nix_develop_with_arg_nixpkg() {
        let args = args![ "eza", "--help" ];
        let p = Processer::new(args);

        assert_eq!(p.nix_develop(), "nix develop nixpkgs#eza --help --command fish");
    }

    #[test]
    fn nix_develop_with_shell_specified() {
        let args = args![ "--command", "zsh" ];
        let p = Processer::new(args);

        assert_eq!(p.nix_develop(), "nix develop --command zsh");
    }
}
