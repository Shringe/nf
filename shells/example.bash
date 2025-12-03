nf_wrapper() {
  eval "$(nf --dryrun "$@")"
}

alias nr="nf_wrapper run"
alias ns="nf_wrapper shell"
alias nd="nf_wrapper develop"
