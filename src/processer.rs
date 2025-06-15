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
}
