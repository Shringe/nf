def main [...raw_args: string] {
  let out = cargo run -- --dryrun ...$raw_args

  let split = $out | split words
  let command = $split | first
  let args = $split | skip

  match $command {
    "nix" => {
      print "run the nix"
    }

    "exec" => {print "run the exec"}

    _ => {
      print "Unrecognized or supported command, exiting."
      exit 1
    }
  }
}
