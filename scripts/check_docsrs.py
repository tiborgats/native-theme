#!/usr/bin/env python3
"""Document a workspace crate the way docs.rs will, before it is published.

docs.rs builds a crate's documentation only after the crate is on crates.io,
so its result cannot be known before a release. What it does can be: it runs
nightly rustdoc on the library, with the features, targets and extra flags of
the crate's `[package.metadata.docs.rs]` table, with `DOCS_RS=1` in the
environment and `--cfg docsrs` given to rustdoc
(https://docs.rs/about/metadata, https://docs.rs/about/builds). This script
does the same for one crate, so a failure that only docs.rs's configuration
reaches -- a nightly-only rustdoc diagnostic, an item behind a feature only
`all-features` turns on, a target-gated item -- stops the release instead of
appearing on docs.rs after it.

Run from the repository root:

    python3 scripts/check_docsrs.py <crate>

It documents the default target first (`default-target`, else the first of
`targets`, else x86_64-unknown-linux-gnu) and then every other listed target,
with `-D warnings` (stricter than docs.rs, which only reports them: a broken
intra-doc link is a defect here). A metadata key this script does not know
fails the check rather than being ignored, so the emulation cannot silently
drift from what docs.rs is told to do.

What it does not reproduce: the dependency versions docs.rs resolves for the
published package (this documents against the workspace's Cargo.lock; the
nightly dependency canary documents against the newest releases), and
docs.rs's sandbox limits on build time and memory.

Exit 0 when every target documented cleanly, 1 otherwise.
"""

import json
import os
import subprocess
import sys

KNOWN_KEYS = {
    "all-features",
    "features",
    "no-default-features",
    "default-target",
    "targets",
    "rustdoc-args",
    "rustc-args",
    "cargo-args",
}
HOST_DEFAULT = "x86_64-unknown-linux-gnu"
TARGET_DIR = os.path.join("target", "docsrs")


def fail(message: str) -> None:
    print(f"check_docsrs: {message}", file=sys.stderr)
    sys.exit(1)


def docsrs_metadata(crate: str) -> dict:
    out = subprocess.run(
        ["cargo", "metadata", "--no-deps", "--format-version", "1"],
        check=False,
        capture_output=True,
        text=True,
    )
    if out.returncode != 0:
        fail(f"cargo metadata failed:\n{out.stderr}")
    for package in json.loads(out.stdout)["packages"]:
        if package["name"] == crate:
            return ((package.get("metadata") or {}).get("docs") or {}).get("rs") or {}
    fail(f"{crate} is not a workspace package")
    return {}


def targets_of(meta: dict) -> list:
    targets = list(meta.get("targets") or [])
    default = meta.get("default-target") or (targets[0] if targets else HOST_DEFAULT)
    return [default] + [t for t in targets if t != default]


def installed_nightly_targets() -> set:
    out = subprocess.run(
        ["rustup", "+nightly", "target", "list", "--installed"],
        check=False,
        capture_output=True,
        text=True,
    )
    if out.returncode != 0:
        fail("the nightly toolchain is not installed (rustup toolchain install nightly)")
    return set(out.stdout.split())


def main() -> None:
    if len(sys.argv) != 2:
        fail("usage: check_docsrs.py <crate>")
    crate = sys.argv[1]
    meta = docsrs_metadata(crate)
    unknown = sorted(set(meta) - KNOWN_KEYS)
    if unknown:
        fail(f"{crate}: [package.metadata.docs.rs] has keys this script does not emulate: {unknown}")

    feature_args = []
    if meta.get("all-features"):
        feature_args.append("--all-features")
    if meta.get("no-default-features"):
        feature_args.append("--no-default-features")
    if meta.get("features"):
        feature_args += ["--features", ",".join(meta["features"])]
    cargo_args = list(meta.get("cargo-args") or [])
    rustdoc_args = ["--cfg", "docsrs", "-D", "warnings"] + list(meta.get("rustdoc-args") or [])

    env = dict(os.environ)
    env["DOCS_RS"] = "1"
    if meta.get("rustc-args"):
        env["RUSTFLAGS"] = " ".join(meta["rustc-args"])

    targets = targets_of(meta)
    installed = installed_nightly_targets()
    missing = [t for t in targets if t not in installed]
    if missing:
        fail(
            f"{crate}: docs.rs documents {', '.join(missing)}, whose nightly standard library is "
            f"not installed (rustup +nightly target add {' '.join(missing)})"
        )

    failed = []
    for target in targets:
        cmd = (
            ["cargo", "+nightly", "rustdoc", "-p", crate, "--lib", "--locked",
             "--target", target, "--target-dir", TARGET_DIR]
            + feature_args
            + cargo_args
            + ["--"]
            + rustdoc_args
        )
        print(f"check_docsrs: {crate} [{target}]: {' '.join(cmd)}", flush=True)
        if subprocess.run(cmd, env=env, check=False).returncode != 0:
            failed.append(target)
    if failed:
        fail(f"{crate}: docs.rs-style documentation failed for {', '.join(failed)}")
    print(f"check_docsrs: {crate}: documented for {', '.join(targets)}")


if __name__ == "__main__":
    main()
