def --env nf_wrapper [...raw_args: string] {
  let out = nf --dryrun ...$raw_args

  let split = $out | split row " "
  let command = $split | first
  let args = $split | skip

  match $command {
    "nix" => { nix ...$args }
    "exec" => { exec ...$args }
    _ => {
      print $"Unrecognized command \"($command)\". Exiting."
      exit 1
    }
  }
}

alias nr = nf_wrapper run
alias ns = nf_wrapper shell
alias nd = nf_wrapper develop
