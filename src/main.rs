mod processer;

use std::{ env, io };
use std::process::{exit, Command, Stdio};

use processer::Processer;

const HELP: &str = "Usage: nf <MODE> <cmd>";

fn main() {
    let mut cmd_args: Vec<String> = env::args().collect();
    if cmd_args.len() < 2 {
        quit();
    }

    // Remove one for `nf` binary and one for the selected mode
    let _ = cmd_args.remove(0);
    let mode = cmd_args.remove(0);

    let processer = Processer::new(cmd_args);
    let cmd = match mode.as_str() {
        "run" => processer.nix_run(),
        "shell" => todo!(),
        "develop" => todo!(),
        _ => quit(),
    };

    println!("{}", cmd);
    let _ = execute_to_stdout(&cmd);
}

fn quit() -> ! {
    println!("{}", HELP);
    exit(1);
}

fn execute_to_stdout(cmd: &str) -> io::Result<()> {
    Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;
    Ok(())
}

