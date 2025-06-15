pub struct Processer {
    args: Vec<String>,
    shell: String,
}

impl Processer {
    #[inline]
    pub fn new(args: Vec<String>, shell: String) -> Self {
        Self {
            args,
            shell,
        }
    }

    /// pkg -> nixpkgs#pkg
    /// Avoids treating args as pkgs
    #[inline]
    fn format_nixpkg(pkg: &str) -> String {
        if pkg.starts_with('-') {
            pkg.to_string()
        } else {
            format!("nixpkgs#{}", pkg)
        }
    }

    /// Check if args contain a specific flag
    #[inline]
    fn contains_flag(&self, flag: &str) -> bool {
        self.args.iter().any(|arg| arg == flag)
    }

    #[inline]
    pub fn nix_run(&self) -> Vec<String> {
        if self.args.is_empty() {
            return vec!["nix".to_string(), "run".to_string()];
        }

        let mut out = Vec::with_capacity(self.args.len() + 4); // Pre-allocate
        out.push("nix".to_string());
        out.push("run".to_string());
        
        out.push(Self::format_nixpkg(&self.args[0]));
        
        if self.args.len() > 1 {
            if !self.contains_flag("--") {
                out.push("--".to_string());
            }

            out.extend_from_slice(&self.args[1..]);
        }
        
        out
    }

    #[inline]
    pub fn nix_shell(&self) -> Vec<String> {
        let mut out = Vec::with_capacity(self.args.len() + 4); // Pre-allocate
        out.push("nix".to_string());
        out.push("shell".to_string());
        
        if !self.args.is_empty() {
            out.push(Self::format_nixpkg(&self.args[0]));
            out.extend_from_slice(&self.args[1..]);
        }
        
        if !self.contains_flag("--command") {
            out.push("--command".to_string());
            out.push(self.shell.clone());
        }
        
        out
    }

    #[inline]
    pub fn nix_develop(&self) -> Vec<String> {
        let mut out = Vec::with_capacity(self.args.len() + 4); // Pre-allocate
        out.push("nix".to_string());
        out.push("develop".to_string());
        
        if !self.args.is_empty() {
            out.push(Self::format_nixpkg(&self.args[0]));
            out.extend_from_slice(&self.args[1..]);
        }
        
        if !self.contains_flag("--command") {
            out.push("--command".to_string());
            out.push(self.shell.clone());
        }
        
        out
    }
}

#[cfg(test)]
mod tests {
    use super::Processer;

    const SHELL: &str = "zsh";

    /// Converts vec![ "a" "b" "c" ] to vec![ "a".to_string() "b".to_string() "c".to_string() ]
    macro_rules! args {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }
    
    #[test]
    fn nix_run() {
        let args = args![];
        let p = Processer::new(args, SHELL.to_string());
        assert_eq!(p.nix_run(), args!["nix", "run"]);
    }
    
    #[test]
    fn nix_run_nixpkg() {
        let args = args!["eza"];
        let p = Processer::new(args, SHELL.to_string());
        assert_eq!(p.nix_run(), args!["nix", "run", "nixpkgs#eza"]);
    }
    
    #[test]
    fn nix_run_with_arg_nixpkg() {
        let args = args!["eza", "to_nix_run", "--"];
        let p = Processer::new(args, SHELL.to_string());
        assert_eq!(p.nix_run(), args!["nix", "run", "nixpkgs#eza", "to_nix_run", "--"]);
    }
    
    #[test]
    fn nix_run_nixpkg_with_arg() {
        let args = args!["eza", "to_command"];
        let p = Processer::new(args, SHELL.to_string());
        assert_eq!(p.nix_run(), args!["nix", "run", "nixpkgs#eza", "--", "to_command"]);
    }
    
    #[test]
    fn nix_run_nixpkg_with_arg_redundant() {
        let args = args!["eza", "--", "to_command"];
        let p = Processer::new(args, SHELL.to_string());
        assert_eq!(p.nix_run(), args!["nix", "run", "nixpkgs#eza", "--", "to_command"]);
    }
    
    #[test]
    fn nix_run_with_arg_nixpkg_with_arg() {
        let args = args!["eza", "to_nix_run", "--", "to_command"];
        let p = Processer::new(args, SHELL.to_string());
        assert_eq!(p.nix_run(), args!["nix", "run", "nixpkgs#eza", "to_nix_run", "--", "to_command"]);
    }
    
    #[test]
    fn nix_shell() {
        let args = args![];
        let p = Processer::new(args, SHELL.to_string());
        assert_eq!(p.nix_shell(), args!["nix", "shell", "--command", SHELL]);
    }
    
    #[test]
    fn nix_shell_nixpkg() {
        let args = args!["eza"];
        let p = Processer::new(args, SHELL.to_string());
        assert_eq!(p.nix_shell(), args!["nix", "shell", "nixpkgs#eza", "--command", SHELL]);
    }
    
    #[test]
    fn nix_shell_with_arg() {
        let args = args!["--help"];
        let p = Processer::new(args, SHELL.to_string());
        assert_eq!(p.nix_shell(), args!["nix", "shell", "--help", "--command", SHELL]);
    }
    
    #[test]
    fn nix_shell_with_arg_nixpkg() {
        let args = args!["eza", "--help"];
        let p = Processer::new(args, SHELL.to_string());
        assert_eq!(p.nix_shell(), args!["nix", "shell", "nixpkgs#eza", "--help", "--command", SHELL]);
    }
    
    #[test]
    fn nix_shell_with_shell_specified() {
        let args = args!["--command", "bash"];
        let p = Processer::new(args, SHELL.to_string());
        assert_eq!(p.nix_shell(), args!["nix", "shell", "--command", "bash"]);
    }
    
    #[test]
    fn nix_develop() {
        let args = args![];
        let p = Processer::new(args, SHELL.to_string());
        assert_eq!(p.nix_develop(), args!["nix", "develop", "--command", SHELL]);
    }
    
    #[test]
    fn nix_develop_nixpkg() {
        let args = args!["eza"];
        let p = Processer::new(args, SHELL.to_string());
        assert_eq!(p.nix_develop(), args!["nix", "develop", "nixpkgs#eza", "--command", SHELL]);
    }
    
    #[test]
    fn nix_develop_with_arg() {
        let args = args!["--help"];
        let p = Processer::new(args, SHELL.to_string());
        assert_eq!(p.nix_develop(), args!["nix", "develop", "--help", "--command", SHELL]);
    }
    
    #[test]
    fn nix_develop_with_arg_nixpkg() {
        let args = args!["eza", "--help"];
        let p = Processer::new(args, SHELL.to_string());
        assert_eq!(p.nix_develop(), args!["nix", "develop", "nixpkgs#eza", "--help", "--command", SHELL]);
    }
    
    #[test]
    fn nix_develop_with_shell_specified() {
        let args = args!["--command", "bash"];
        let p = Processer::new(args, SHELL.to_string());
        assert_eq!(p.nix_develop(), args!["nix", "develop", "--command", "bash"]);
    }
}
