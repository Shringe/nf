## TODO:
### Commands
- [x] nf config add
- [x] nf config remove
### Functionality
- [x] Add analyze ability that can unprocess nf expansions for demonstration
- [x] implement nf config file with default shell override
- [x] Rework templates to take save multiple files instead of just the flake.nix
- [x] Add ability for nf shell/develop to look for ./flake/flake.nix and ./flake/flake.lock as well. The subdirectories should probably be configurable
- [ ] Add option for avoiding nested devshells/shells, e.x. if you are already in a shell, then "exit ; nix develop", instead of entering a second layer deep with "nix develop"
### Other
- [ ] Add readme about
- [x] Better clap documentation
