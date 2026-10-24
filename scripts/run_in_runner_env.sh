#!/usr/bin/env bash
# Runs `cargo <args>` with each test binary in the environment of CI's Linux
# runner, so that a test which assumes this host's desktop, icon themes or fonts
# fails here before it fails on CI.
#
#   scripts/run_in_runner_env.sh test -p native-theme-egui --example showcase-egui
#   scripts/run_in_runner_env.sh --prepare
#
# `--prepare` builds the environment and runs nothing. It, and a run, exit 75
# (EX_TEMPFAIL) only where a package is not in target/runner-env/debs with its
# SHA-256 and neither server delivers it, one or both of them out of reach
# (no connection, a timeout, a transfer closed early) or answering
# transiently (5xx, 408, 429), so a caller can tell an outage from a failure;
# where the other server answered 4xx the message says the pin may have
# rotted, which only both servers' 4xx decides. Everything else fails with 1,
# naming its cause: a 4xx answer from both servers (the pin has rotted; the
# message names the package and both answers), a file that cannot be written
# or extracted, and a downloaded package whose SHA-256 differs. curl gives up
# on a connection after 10 s and on a transfer slower than 1 KiB/s for 30 s,
# so an outage cannot hold the run.
#
# Cargo builds as usual; its target runner for the host's target
# (`--config target.<host>.runner`, which no configuration file overrides)
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
# - An empty system configuration directory (target/runner-env/config), the
#   one entry of XDG_CONFIG_DIRS: unset, the KDE reader would read this
#   host's /etc/xdg/kdeglobals, where the host has one, as KConfig does
#   (native-theme/src/kde/mod.rs, kdeglobals_files_pure), and the runner has
#   none.
# - A PATH of the programs the binaries run that the runner has: fc-match
#   (fontconfig; the run's substitutes were fc-match's, DejaVu Sans), sh, sleep,
#   true, and the toolchain's cargo and rustc. Not gsettings, xrdb, xrandr or
#   xfconf-query, the desktop readers native_theme::detect asks.
# - The runner's icon themes: Adwaita, hicolor, Humanity and ubuntu-mono,
#   none other (no breeze). The image installs firefox from the mozillateam
#   PPA (install-firefox.sh), which Depends on libgtk-3-0t64; it Depends on
#   adwaita-icon-theme and hicolor-icon-theme, adwaita-icon-theme on
#   ubuntu-mono, and ubuntu-mono on humanity-icon-theme (the PPA's and Ubuntu
#   noble's Packages indexes). Run 40041829633 fits it: its expander test
#   failed for kde-breeze at 1.5 and not for adwaita, which fails too where no
#   icon theme is installed at all.
#   They are pinned below with the fonts and extracted into
#   target/runner-env/root/usr/share, the one entry of XDG_DATA_DIRS: the icon
#   lookup and the installed-theme list read their directories from it, the
#   empty home's ~/.icons and ~/.local/share (native-theme/src/icons.rs:1111-
#   1150; freedesktop-icons 0.4.0 src/theme/paths.rs:12-34 through xdg 2.5.2,
#   which takes /usr/local/share:/usr/share where it is unset,
#   src/base_directories.rs:306-309).
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
#   from snapshot.ubuntu.com at the image's date, so the URLs stay valid (else
#   from archive.ubuntu.com's pool; the SHA-256 is checked either way), into
#   target/runner-env/debs once, with the icon themes; a later run reuses
#   them. FONTCONFIG_FILE names a
#   configuration whose one font directory is theirs: fontdb, which
#   native-theme's `system-fonts` loads, reads it (fontdb 0.23.0
#   src/lib.rs:509-510, its <dir> entries at :560-570), and so does fc-match,
#   which native_theme::fonts::substitute_family runs
#   (native-theme/src/fonts.rs:357-360).
#
# Requires curl, sha256sum, bsdtar (libarchive) and fontconfig's fc-match.
set -euo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
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
        XDG_DATA_DIRS="$ENV_DIR/root/usr/share" \
        XDG_CONFIG_DIRS="$ENV_DIR/config" \
        DBUS_SESSION_BUS_ADDRESS="unix:path=$ENV_DIR/runtime/no-bus" \
        FONTCONFIG_FILE="$ENV_DIR/fonts.conf" \
        "$@"
fi

cd "$ROOT"

PREPARE=""
if [ "${1:-}" = "--prepare" ]; then
    PREPARE=1
    shift
fi
if [[ ! $PREPARE ]] && [ "$#" -eq 0 ]; then
    printf 'usage: scripts/run_in_runner_env.sh [--prepare | <cargo arguments>]\n' >&2
    exit 2
fi

# Fetches $1 into $2. Returns 0 when fetched; 75 for an outage, with
# FETCHED saying what happened: curl could not resolve, connect, finish the
# TLS handshake or receive in time, the transfer closed early, or the server
# answered 5xx, 408 (request timeout) or 429 (too many requests), each
# transient, curl retrying them; 2 when the server answered another 4xx, the
# file not there; and 1, having printed the cause, for anything else (a file
# it cannot write among them).
FETCHED=""
fetch() {
    local url=$1 deb=$2 code status=0
    code=$(curl -sSL --connect-timeout 10 --speed-limit 1024 --speed-time 30 \
        --max-time 600 --retry 2 --retry-max-time 120 -o "$deb" -w '%{http_code}' "$url") \
        || status=$?
    case "$status" in
        0) ;;
        5 | 6 | 7 | 18 | 28 | 35 | 52 | 55 | 56)
            FETCHED="$url is out of reach (curl exit $status)"
            return 75
            ;;
        23)
            printf 'run_in_runner_env.sh: cannot write %s (curl exit 23)\n' "$deb" >&2
            return 1
            ;;
        *)
            printf 'run_in_runner_env.sh: curl exit %s for %s\n' "$status" "$url" >&2
            return 1
            ;;
    esac
    case "$code" in
        2??) return 0 ;;
        5?? | 408 | 429)
            FETCHED="$url answered HTTP $code, a transient answer"
            return 75
            ;;
        4??)
            FETCHED="$url answered HTTP $code"
            return 2
            ;;
        *)
            printf 'run_in_runner_env.sh: HTTP %s for %s\n' "$code" "$url" >&2
            return 1
            ;;
    esac
}

for tool in curl sha256sum bsdtar fc-match sh sleep true; do
    if ! type -P "$tool" >/dev/null; then
        printf 'run_in_runner_env.sh: %s is not installed\n' "$tool" >&2
        exit 1
    fi
done

SNAPSHOT="https://snapshot.ubuntu.com/ubuntu/20261012T000000Z"
ARCHIVE="http://archive.ubuntu.com/ubuntu"
PACKAGES=(
    "9df0579cd298a1f2b7a40efd519c849745aecd193a09733cf4d1032284a62bbf pool/main/f/fontconfig/fontconfig-config_2.15.0-1.1ubuntu2_amd64.deb"
    "40049660c194f3b8a2541fc7369efebb10e9f94bdac836a2f38fafedd10fa73a pool/main/f/fonts-dejavu/fonts-dejavu-core_2.37-8_all.deb"
    "8a599d6553307db7ecb795d2f0e5a301e03234afc75c7358b0ba43466454c89a pool/main/f/fonts-dejavu/fonts-dejavu-mono_2.37-8_all.deb"
    "5502e5ccc416683e5251dc105b37be31158bc2e02133221e56524ea2a8e426fe pool/main/f/fonts-dejavu/fonts-dejavu-extra_2.37-8_all.deb"
    "065c2ab1abc9108b17d401016dc594b79750904390f095845c93bb06e1153acc pool/main/f/fonts-liberation/fonts-liberation_2.1.5-3_all.deb"
    "9c64f03daaa4b0eee8b36ba53e4ce8ac22ec12437951c699197344989c7dc2dc pool/main/f/fonts-liberation-sans-narrow/fonts-liberation-sans-narrow_1.07.6-4_all.deb"
    "b102b62ce6a7313315223cff5e052bdf8fc0ad162c9e961ce0dc2202bc139ce2 pool/main/f/fonts-noto-color-emoji/fonts-noto-color-emoji_2.047-0ubuntu0.24.04.1_all.deb"
    "530d1991a87ffda50f35f9a6d23712ed04c10b742c3e434097840fed50b1eb4e pool/main/a/adwaita-icon-theme/adwaita-icon-theme_46.0-1_all.deb"
    "9000eb98868252261978ff49501c0ace3124cb369c395e9f5015ddc556fe2ba6 pool/main/h/hicolor-icon-theme/hicolor-icon-theme_0.17-2_all.deb"
    "a9bb76c7eb926bd97e50455a79f266de826a246b2250e1ac4aebcda013dd51c7 pool/main/h/humanity-icon-theme/humanity-icon-theme_0.6.16_all.deb"
    "3a58114f10170b253d2a574db854fa1c4399565888c78f28bbe53ce889894cdb pool/main/u/ubuntu-themes/ubuntu-mono_24.04-0ubuntu1_all.deb"
)
# The environment is built again whenever this list changes.
STAMP=$(printf '%s\n' "${PACKAGES[@]}")

if [ "$(cat "$ENV_DIR/.complete" 2>/dev/null)" != "$STAMP" ]; then
    # Everything but the downloaded packages is built again; a package whose
    # file is already there with its SHA-256 is not downloaded again.
    mkdir -p "$ENV_DIR"
    find "$ENV_DIR" -mindepth 1 -maxdepth 1 ! -name debs -exec rm -rf {} +
    mkdir -p "$ENV_DIR/debs" "$ENV_DIR/root" "$ENV_DIR/fonts" "$ENV_DIR/conf.d" \
        "$ENV_DIR/cache" "$ENV_DIR/bin" "$ENV_DIR/runtime"
    for entry in "${PACKAGES[@]}"; do
        sha=${entry%% *}
        path=${entry#* }
        deb="$ENV_DIR/debs/${path##*/}"
        if ! printf '%s  %s\n' "$sha" "$deb" | sha256sum --check --quiet --status 2>/dev/null; then
            # The snapshot first, then the archive's own pool, which holds the
            # same file while the version is current: the SHA-256 decides.
            snapshot=0
            archive=0
            FETCHED=""
            fetch "$SNAPSHOT/$path" "$deb" || snapshot=$?
            from_snapshot=$FETCHED
            if [ "$snapshot" -eq 1 ]; then
                rm -f "$deb"
                exit 1
            elif [ "$snapshot" -ne 0 ]; then
                FETCHED=""
                fetch "$ARCHIVE/$path" "$deb" || archive=$?
            fi
            if [ "$snapshot" -ne 0 ] && [ "$archive" -ne 0 ]; then
                rm -f "$deb"
                if [ "$archive" -eq 1 ]; then
                    exit 1
                elif [ "$snapshot" -eq 2 ] && [ "$archive" -eq 2 ]; then
                    printf 'run_in_runner_env.sh: %s is gone from both servers: %s; %s; update its pin\n' \
                        "${path##*/}" "$from_snapshot" "$FETCHED" >&2
                    exit 1
                fi
                # At least one server could not answer. Where the other said
                # the file is not there, the pin may have rotted, but only an
                # answer from both decides that: the archive's pool drops a
                # version once it is superseded, and the snapshot may lack a
                # file the pool still holds. So it is an outage, said as such.
                note=""
                if [ "$snapshot" -eq 2 ] || [ "$archive" -eq 2 ]; then
                    note="; the pin may have rotted, but the other server could not confirm it"
                fi
                printf 'run_in_runner_env.sh: cannot fetch %s: %s; %s%s\n' \
                    "${path##*/}" "$from_snapshot" "$FETCHED" "$note" >&2
                exit 75
            fi
            printf '%s  %s\n' "$sha" "$deb" | sha256sum --check --quiet
        fi
        if ! bsdtar -xOf "$deb" 'data.tar.*' | bsdtar -xf - -C "$ENV_DIR/root"; then
            printf 'run_in_runner_env.sh: cannot extract %s into %s\n' "$deb" "$ENV_DIR/root" >&2
            exit 1
        fi
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
    printf '%s\n' "$STAMP" >"$ENV_DIR/.complete"
fi
if [[ $PREPARE ]]; then
    exit 0
fi

# The home and the system configuration directory are emptied each run, so
# nothing a test wrote there is read back.
rm -rf "$ENV_DIR/home" "$ENV_DIR/config"
mkdir -p "$ENV_DIR/home" "$ENV_DIR/config"
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
HOST=$(rustc -vV | sed -n 's/^host: //p')
export CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}"
exec cargo --config "target.$HOST.runner = ['$RUNNER', '--runner']" "$@"
