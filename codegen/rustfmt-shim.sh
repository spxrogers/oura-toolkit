#!/usr/bin/env bash
# rustfmt shim used by `just gen-rust`.
#
# cargo-progenitor formats its generated code with UNSTABLE rustfmt options
# (`wrap_comments`, `normalize_doc_attributes`) which (a) require nightly and (b) can corrupt
# doc comments — mangling one produced invalid Rust (a dropped brace) for the Oura spec. This
# shim runs nightly rustfmt with its DEFAULT config instead: it strips progenitor's
# `--config-path=…` and `--unstable-features` args and pins the edition, leaving comment bodies
# untouched. Point RUSTFMT at this script. See issue #6.
set -euo pipefail

args=()
for a in "$@"; do
  case "$a" in
    --config-path=*) ;;      # drop progenitor's unstable rustfmt.toml
    --unstable-features) ;;  # drop (only valid on the unstable config anyway)
    *) args+=("$a") ;;
  esac
done

exec "$(rustup which --toolchain nightly rustfmt)" --edition 2021 "${args[@]}"
