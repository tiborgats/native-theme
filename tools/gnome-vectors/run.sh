#!/bin/bash
# Runs inside the GNOME container (tools/linux-containers/gnome), from the
# repository's root: builds gnome-vectors, writes $1/vectors.tsv, writes it a
# second time and checks that both are byte-identical. run.txt records each
# step's exit status.
set -u
out=$1
build=/tmp/gnome-vectors-build
mkdir -p "$out"
log="$out/run.txt"
note() { echo "$*" | tee -a "$log"; }

meson setup "$build" tools/gnome-vectors >"$out/build.log" 2>&1 && meson compile -C "$build" >>"$out/build.log" 2>&1
status=$?
note "build: exit $status"
if [ $status -ne 0 ]; then cat "$out/build.log"; exit 1; fi

"$build/gnome-vectors" "$out/vectors.tsv"
status=$?
note "gnome-vectors: exit $status"
[ $status -eq 0 ] || exit 1
"$build/gnome-vectors" "$build/vectors-again.tsv"
note "gnome-vectors, second run: exit $?"
if cmp "$out/vectors.tsv" "$build/vectors-again.tsv"; then
  note "two runs byte-identical"
else
  note "two runs differ"
  exit 1
fi
note "rows: $(grep -vc '^#' "$out/vectors.tsv") (with the header line)"
sha256sum "$out/vectors.tsv" | tee -a "$log"
