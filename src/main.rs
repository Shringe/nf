mod processer;

use std::env;
use std::process::{ exit, Command };

use processer::Processer;

const HELP: &str = "Usage: nf <MODE> <cmd>";

fn main() {
    // Remove one for `nf` binary and one for the selected mode
    let mut args = env::args();
    let _ = args.next(); // skip binary name
    let mode = args.next().unwrap_or_else(|| quit());
    let cmd_args: Vec<String> = args.collect();

    let processer = Processer::new(cmd_args);
    let cmd = match mode.as_str() {
        "run" => processer.nix_run(),
        "shell" => processer.nix_shell(),
        "develop" => processer.nix_develop(),
        _ => quit(),
    };

    execute_to_stdout(&cmd);
}

#[inline]
fn quit() -> ! {
    println!("{}", HELP);
    exit(1);
}

/// Faster but UNIX exclusive
#[inline]
#[cfg(unix)]
fn execute_to_stdout(cmd: &[String]) {
    use std::os::unix::process::CommandExt;
    let _ = Command::new(&cmd[0])
        .args(&cmd[1..])
        .exec(); // This replaces the current process
}

#[inline]
#[cfg(not(unix))]
fn execute_to_stdout(cmd: &[String]) {
    use std::process::Stdio;
    let _ = Command::new(&cmd[0]).args(&cmd[1..])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();
}
