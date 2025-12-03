def main [...raw_args: string] {
  let out = cargo run -- --dryrun ...$raw_args

  let split = $out | split row " "
  let command = $split | first
  let args = $split | skip

  print $"raw_args: ($raw_args)"
  print $"args:     ($args)"
  print $"out:      ($out)"
  print $"command:  ($out)"

  match $command {
    "nix" => { nix ...$args }
    "exec" => { exec ...$args }
    _ => {
      print $"Unrecognized or supported command \"($command)\", exiting."
      exit 1
    }
  }
}
