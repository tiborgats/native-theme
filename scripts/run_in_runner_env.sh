#!/usr/bin/env bash
# Runs `cargo <args>` with each test binary in the environment of CI's Linux
# runner, so that a test which assumes this host's desktop or fonts fails here
# before it fails on CI.
#
#   scripts/run_in_runner_env.sh test -p native-theme-egui --example showcase-egui
#
# Cargo builds as usual; its target runner (`--config target.'cfg(all())'.runner`)
# starts each test binary through this script's `--runner` mode, which clears
# the environment and gives the binary what an ubuntu-latest job has:
#
# - No desktop. native_theme::detect recognised none on the runner: run
#   40041829633's showcase tests found adwaita the platform preset ("only this
#   platform's presets run here: default, adwaita, ...") and no system icon
#   theme. So the binary gets no XDG_CURRENT_DESKTOP, DISPLAY or
#   WAYLAND_DISPLAY, and DBUS_SESSION_BUS_ADDRESS names a socket that does not
#   exist: unset, zbus would connect to $XDG_RUNTIME_DIR/bus or
#   /run/user/<uid>/bus (zbus 5.19.0 src/address/mod.rs:67-80), where a desktop
#   session's portal answers.
# - An empty home (target/runner-env/home): no kdeglobals, no dconf database.
#   CARGO_HOME stays this host's cargo home, as the job sets its own
#   (CARGO_HOME: /home/runner/.cargo in the run's test step).
# - A PATH of the programs the binaries run that the runner has: fc-match
#   (fontconfig; the run's substitutes were fc-match's, DejaVu Sans), sh, sleep,
#   true, and the toolchain's cargo and rustc. Not gsettings, xrdb, xrandr or
#   xfconf-query, the desktop readers native_theme::detect asks.
# - The runner's fonts. The job ran on the ubuntu-24.04 image 20261012.334
#   (the run's "Set up job" log), whose fonts are the Ubuntu 24.04 packages
#   pinned below, each with the SHA-256 the Ubuntu archive's Packages index
#   states for it:
#     fonts-noto-color-emoji        the image's own apt list (toolset-2404.json)
#     fonts-liberation              google-chrome-stable Depends on it
#     fonts-liberation-sans-narrow  fonts-liberation Recommends it
#     fonts-dejavu-core, -extra     temurin-*-jdk Recommends them (the image
#                                   installs the JDKs without
#                                   --no-install-recommends)
#     fonts-dejavu-mono             fonts-dejavu-core Depends on it
#   with fontconfig-config's configuration (fontconfig 2.15.0): the files its
#   /etc/fonts/conf.d links name and the DejaVu packages' own, less
#   50-user.conf and 51-local.conf, which would read this host's. They come
#   from snapshot.ubuntu.com at the image's date, so the URLs stay valid, into
#   target/runner-env/ once; a later run reuses them. FONTCONFIG_FILE names a
#   configuration whose one font directory is theirs: fontdb, which
#   native-theme's `system-fonts` loads, reads it (fontdb 0.23.0
#   src/lib.rs:509-510, its <dir> entries at :560-570), and so does fc-match,
#   which native_theme::fonts::substitute_family runs
#   (native-theme/src/fonts.rs:357-360).
#
# Requires curl, sha256sum and bsdtar (libarchive).
set -euo pipefail

ROOT=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
ENV_DIR="$ROOT/target/runner-env"

if [ "${1:-}" = "--runner" ]; then
    shift
    # What cargo sets for a test binary (CARGO_MANIFEST_DIR and the like, the
    # library path), the toolchain's own (CARGO_HOME, which the job sets, and
    # rustup's, so a test that runs cargo finds its registry) and the test
    # harness's settings pass; nothing else.
    keep=()
    for name in $(compgen -e); do
        case "$name" in
            CARGO | CARGO_* | RUSTUP_* | RUST_BACKTRACE | RUST_TEST_* | LD_LIBRARY_PATH)
                keep+=("$name=${!name}")
                ;;
        esac
    done
    exec env -i "${keep[@]}" \
        HOME="$ENV_DIR/home" \
        PATH="$ENV_DIR/bin" \
        XDG_RUNTIME_DIR="$ENV_DIR/runtime" \
        DBUS_SESSION_BUS_ADDRESS="unix:path=$ENV_DIR/runtime/no-bus" \
        FONTCONFIG_FILE="$ENV_DIR/fonts.conf" \
        "$@"
fi

cd "$ROOT"

if [ "$#" -eq 0 ]; then
    printf 'usage: scripts/run_in_runner_env.sh <cargo arguments>\n' >&2
    exit 2
fi

for tool in curl sha256sum bsdtar fc-match sh sleep true; do
    if ! type -P "$tool" >/dev/null; then
        printf 'run_in_runner_env.sh: %s is not installed\n' "$tool" >&2
        exit 1
    fi
done

SNAPSHOT="https://snapshot.ubuntu.com/ubuntu/20261012T000000Z"
PACKAGES=(
    "9df0579cd298a1f2b7a40efd519c849745aecd193a09733cf4d1032284a62bbf pool/main/f/fontconfig/fontconfig-config_2.15.0-1.1ubuntu2_amd64.deb"
    "40049660c194f3b8a2541fc7369efebb10e9f94bdac836a2f38fafedd10fa73a pool/main/f/fonts-dejavu/fonts-dejavu-core_2.37-8_all.deb"
    "8a599d6553307db7ecb795d2f0e5a301e03234afc75c7358b0ba43466454c89a pool/main/f/fonts-dejavu/fonts-dejavu-mono_2.37-8_all.deb"
    "5502e5ccc416683e5251dc105b37be31158bc2e02133221e56524ea2a8e426fe pool/main/f/fonts-dejavu/fonts-dejavu-extra_2.37-8_all.deb"
    "065c2ab1abc9108b17d401016dc594b79750904390f095845c93bb06e1153acc pool/main/f/fonts-liberation/fonts-liberation_2.1.5-3_all.deb"
    "9c64f03daaa4b0eee8b36ba53e4ce8ac22ec12437951c699197344989c7dc2dc pool/main/f/fonts-liberation-sans-narrow/fonts-liberation-sans-narrow_1.07.6-4_all.deb"
    "b102b62ce6a7313315223cff5e052bdf8fc0ad162c9e961ce0dc2202bc139ce2 pool/main/f/fonts-noto-color-emoji/fonts-noto-color-emoji_2.047-0ubuntu0.24.04.1_all.deb"
)

if [ ! -f "$ENV_DIR/.complete" ]; then
    rm -rf "$ENV_DIR"
    mkdir -p "$ENV_DIR/debs" "$ENV_DIR/root" "$ENV_DIR/fonts" "$ENV_DIR/conf.d" \
        "$ENV_DIR/cache" "$ENV_DIR/bin" "$ENV_DIR/runtime"
    for entry in "${PACKAGES[@]}"; do
        sha=${entry%% *}
        path=${entry#* }
        deb="$ENV_DIR/debs/${path##*/}"
        curl -fsSL --retry 3 -o "$deb" "$SNAPSHOT/$path"
        printf '%s  %s\n' "$sha" "$deb" | sha256sum --check --quiet
        bsdtar -xOf "$deb" 'data.tar.*' | bsdtar -xf - -C "$ENV_DIR/root"
    done
    find "$ENV_DIR/root/usr/share/fonts" -type f -name '*.ttf' -exec cp {} "$ENV_DIR/fonts/" \;
    # fontconfig-config's links point at /usr/share/fontconfig/conf.avail, the
    # DejaVu packages' at ../conf.avail beside them: each is resolved inside
    # the extracted tree, never on this host.
    for link in "$ENV_DIR/root/etc/fonts/conf.d/"*.conf; do
        name=${link##*/}
        case "$name" in
            50-user.conf | 51-local.conf) continue ;;
        esac
        target=$(readlink "$link")
        case "$target" in
            /*) source="$ENV_DIR/root$target" ;;
            *) source="$ENV_DIR/root/etc/fonts/conf.d/$target" ;;
        esac
        cp "$source" "$ENV_DIR/conf.d/$name"
    done
    cat >"$ENV_DIR/fonts.conf" <<EOF
<?xml version="1.0"?>
<!DOCTYPE fontconfig SYSTEM "urn:fontconfig:fonts.dtd">
<fontconfig>
	<dir>$ENV_DIR/fonts</dir>
	<cachedir>$ENV_DIR/cache</cachedir>
	<include ignore_missing="no">$ENV_DIR/conf.d</include>
</fontconfig>
EOF
    touch "$ENV_DIR/.complete"
fi

# The home is emptied each run, so nothing a test wrote there is read back.
rm -rf "$ENV_DIR/home"
mkdir -p "$ENV_DIR/home"
for tool in fc-match sh sleep true; do
    ln -sf "$(type -P "$tool")" "$ENV_DIR/bin/$tool"
done
# The job's PATH holds the toolchain, and a test may run cargo, which runs
# rustc: the toolchain's own binaries, not rustup's proxies, which would look
# for their settings in the empty home.
SYSROOT=$(rustc --print sysroot)
for tool in cargo rustc; do
    ln -sf "$SYSROOT/bin/$tool" "$ENV_DIR/bin/$tool"
done

RUNNER="$ROOT/scripts/run_in_runner_env.sh"
export CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}"
exec cargo --config "target.'cfg(all())'.runner = ['$RUNNER', '--runner']" "$@"
