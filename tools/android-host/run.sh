#!/bin/bash
# Runs on an ubuntu runner with KVM, from the repository's root: installs the
# pinned Android SDK packages into a fresh SDK folder (each archive fetched by
# its exact name from Google's repository, its SHA-1 checked against the
# repository's and its SHA-256 against the one recorded at the first fetch;
# never sdkmanager, which installs only a package's newest revision), builds the host (the pinned Gradle wrapper,
# AGP and Kotlin of gradle/libs.versions.toml), then twice: boots a fresh AVD
# of the chosen system image (cold boot, wiped data, no window, SwiftShader),
# runs CaptureTest (one test per specimen) and pulls its PNGs and TOML
# fragments, merged into index.toml; and checks that the two
# runs wrote byte-identical files (sha256sum).
#
#     bash tools/android-host/run.sh <image> <out>
#
# <image> is 37.1 (Android 17 QPR1, 16 KB pages only) or 37.0 (Android 17,
# 4 KB pages). Writes into <out>:
#   index.toml        the system image, emulator, SDK packages, device build,
#                     Compose and build versions, the runner
#   sdk.txt           each SDK archive: package path, revision, name, SHA-1, SHA-256
#   build.txt, dependencies.txt, build-environment.txt, gradle-version.txt
#                     Gradle's output; accel-check.txt, the emulator's KVM check
#   run<N>/capture/   the PNGs and index.toml (device.toml and each test's
#                     fragment, merged)
#   run<N>/           emulator.txt, instrument.txt, getprop.txt, logcat.txt
#   run.txt           each step's exit status, the comparison
#   sha256.txt        every PNG's SHA-256 (run 1)
set -u
image=$1
out=$2
mkdir -p "$out"
out=$(cd "$out" && pwd)
host=$(cd "$(dirname "$0")" && pwd)
log="$out/run.txt"
: >"$log"
note() { echo "$*" | tee -a "$log"; }
manifest() { (cd "$1" && find . -type f -print | sort | xargs sha256sum); }
fail() { note "$*"; chmod -R a+rX "$out"; exit 1; }

repository=https://dl.google.com/android/repository
case "$image" in
  37.1)
    image_path="system-images;android-37.1;google_apis_ps16k;x86_64" image_revision=9
    image_archive=sys-img/google_apis/x86_64-ps16k-37.1_r09.zip
    image_sha1=35c0da1cd298f3426a5b48233f59515097446a28
    image_sha256=11b9682183827379e8d415db880a30b9ec84d4032650e08e6149b69b30d545a0 ;;
  37.0)
    image_path="system-images;android-37.0;google_apis;x86_64" image_revision=8
    image_archive=sys-img/google_apis/x86_64-37.0_r08.zip
    image_sha1=a266cfbcf4d6c769b166c0af4d68eff0e4f4a5e3
    image_sha256= ;;
  *) fail "unknown image $image (37.1 or 37.0)" ;;
esac

# package path | revision | archive | SHA-1 (repository2-3.xml, read 2026-10-30)
# | SHA-256 (run 40981395261; empty: recorded, not checked)
packages=(
  "platform-tools|37.0.1|platform-tools_r37.0.1-linux.zip|477254aa5f903c15cf51001717bdf347fb6b53e0|d230f13842f60f782a8645f9c813f8f845bf36089ea7289f28c48f17979313f1"
  "platforms;android-37.1|1|platform-37.1_r01.zip|62aeba1ec09c2b1fdf888e79549daf678fa0d75f|cadf0a541847820ea3d8ffc5c192562a18376cf9ba510bf9659c772f9a442184"
  "build-tools;37.0.0|37.0.0|build-tools_r37_linux.zip|70954e99f4c3d9d46ee70fa32624672fe7cd6ebe|01af179347cbcd9c208b7f8171f7b21f6dd1d2f85bcd15e88caa51d5d7b86060"
  "emulator|37.2.13|emulator-linux_x64-16740729.zip|f41f9e31eb5ca64926603c03265c50bbc0ff4172|cbc7c3cfb83220b8a0f9c513fdab557712e74d339ec19ddffa43f23350ae9bdd"
  "$image_path|$image_revision|$image_archive|$image_sha1|$image_sha256"
)

# The runner's own SDK gives only its accepted licences.
runner_sdk=${ANDROID_HOME:-}
sdk=${RUNNER_TEMP:-/tmp}/android-sdk
rm -rf "$sdk"
mkdir -p "$sdk"
if [ -n "$runner_sdk" ] && [ -d "$runner_sdk/licenses" ]; then cp -r "$runner_sdk/licenses" "$sdk/licenses"; fi
export ANDROID_HOME=$sdk ANDROID_SDK_ROOT=$sdk ANDROID_AVD_HOME=$sdk/avd

note "cpu: $(grep -m1 '^model name' /proc/cpuinfo | cut -d: -f2-) ($(nproc) CPUs)"
note "runner: ${ImageOS:-?} ${ImageVersion:-?}"
note "kvm: $(ls -l /dev/kvm 2>&1)"
note "java: $(java -version 2>&1 | head -n 1)"
df -h / "$sdk" | tee -a "$log"

: >"$out/sdk.txt"
for entry in "${packages[@]}"; do
  IFS='|' read -r path revision archive sha1 pinned_sha256 <<<"$entry"
  zip="$sdk/download.zip"
  curl -sSfL --retry 3 -o "$zip" "$repository/$archive" || fail "download $archive: failed"
  got_sha1=$(sha1sum "$zip" | cut -d' ' -f1)
  sha256=$(sha256sum "$zip" | cut -d' ' -f1)
  [ "$got_sha1" = "$sha1" ] || fail "$archive: SHA-1 $got_sha1, expected $sha1"
  [ -z "$pinned_sha256" ] || [ "$sha256" = "$pinned_sha256" ] || fail "$archive: SHA-256 $sha256, expected $pinned_sha256"
  echo "$path	$revision	$archive	sha1:$sha1	sha256:$sha256" >>"$out/sdk.txt"
  target="$sdk/${path//;//}"
  unpack=$(mktemp -d "$sdk/unpack.XXXXXX")
  unzip -q "$zip" -d "$unpack" || fail "unzip $archive: failed"
  rm -f "$zip"
  top=$(find "$unpack" -mindepth 1 -maxdepth 1)
  [ "$(echo "$top" | wc -l)" = 1 ] || fail "$archive: not one top-level folder"
  mkdir -p "$(dirname "$target")"
  mv "$top" "$target"
  rmdir "$unpack"
  installed=$(grep -m1 '^Pkg.Revision=' "$target/source.properties" | cut -d= -f2)
  note "$path: $archive, Pkg.Revision $installed"
done
df -h / | tee -a "$log"

cd "$host" || fail "no $host"
./gradlew --no-daemon --version >"$out/gradle-version.txt" 2>&1
./gradlew --no-daemon :app:assembleDebug :app:assembleDebugAndroidTest >"$out/build.txt" 2>&1
status=$?
note "gradlew assembleDebug assembleDebugAndroidTest: exit $status"
[ $status -eq 0 ] || { tail -n 60 "$out/build.txt"; fail "build failed"; }
for configuration in debugRuntimeClasspath debugAndroidTestRuntimeClasspath; do
  ./gradlew --no-daemon :app:dependencies --configuration "$configuration" >>"$out/dependencies.txt" 2>&1
done
./gradlew --no-daemon buildEnvironment >"$out/build-environment.txt" 2>&1
apk=$host/app/build/outputs/apk/debug/app-debug.apk
test_apk=$host/app/build/outputs/apk/androidTest/debug/app-debug-androidTest.apk
[ -f "$apk" ] && [ -f "$test_apk" ] || fail "APKs missing"
cd - >/dev/null || exit 1

adb=$sdk/platform-tools/adb
emulator=$sdk/emulator/emulator
"$emulator" -accel-check >"$out/accel-check.txt" 2>&1
note "accel-check: $(tr '\n' ' ' <"$out/accel-check.txt")"

image_dir="${image_path//;//}"
image_tag=$(echo "$image_path" | cut -d';' -f3)
avd=$ANDROID_AVD_HOME/host.avd
failed=0
for run in 1 2; do
  dir="$out/run$run"
  mkdir -p "$dir/capture"
  rm -rf "$ANDROID_AVD_HOME"
  mkdir -p "$avd"
  printf 'avd.ini.encoding=UTF-8\npath=%s\ntarget=android-%s\n' "$avd" "$image" >"$ANDROID_AVD_HOME/host.ini"
  cat >"$avd/config.ini" <<EOF
avd.ini.encoding=UTF-8
abi.type=x86_64
hw.cpu.arch=x86_64
hw.cpu.ncore=4
hw.ramSize=4096
disk.dataPartition.size=6G
image.sysdir.1=$image_dir/
tag.id=$image_tag
PlayStore.enabled=false
hw.lcd.width=720
hw.lcd.height=1280
hw.lcd.density=320
hw.gpu.enabled=yes
hw.gpu.mode=swiftshader_indirect
hw.keyboard=yes
hw.mainKeys=no
hw.camera.back=none
hw.camera.front=none
hw.audioInput=no
hw.audioOutput=no
fastboot.forceColdBoot=yes
EOF
  cp "$avd/config.ini" "$dir/config.ini"

  "$emulator" -avd host -no-window -no-audio -no-boot-anim -no-snapshot -wipe-data \
    -gpu swiftshader_indirect -accel on -no-metrics >"$dir/emulator.txt" 2>&1 &
  emulator_pid=$!
  timeout 600 "$adb" wait-for-device || fail "run $run: no device in 600 s"
  start=$SECONDS
  until [ "$("$adb" shell getprop sys.boot_completed 2>/dev/null | tr -d '\r')" = 1 ]; do
    if ! kill -0 "$emulator_pid" 2>/dev/null; then tail -n 40 "$dir/emulator.txt"; fail "run $run: the emulator exited"; fi
    if [ $((SECONDS - start)) -gt 900 ]; then tail -n 40 "$dir/emulator.txt"; fail "run $run: no boot in 900 s"; fi
    sleep 5
  done
  note "run $run: booted in $((SECONDS - start)) s"
  "$adb" shell svc power stayon true
  "$adb" shell settings put system screen_off_timeout 2147483647
  "$adb" shell input keyevent KEYCODE_WAKEUP
  "$adb" shell wm dismiss-keyguard
  "$adb" shell getprop >"$dir/getprop.txt"
  { "$adb" shell getconf PAGE_SIZE; "$adb" shell wm size; "$adb" shell wm density; } >"$dir/display.txt" 2>&1
  "$adb" install -r -t "$apk" >"$dir/install.txt" 2>&1
  "$adb" install -r -t "$test_apk" >>"$dir/install.txt" 2>&1
  note "run $run: install $(tr '\r\n' '  ' <"$dir/install.txt")"
  "$adb" logcat -c
  timeout 2400 "$adb" shell am instrument -w -r -e class dev.nativetheme.androidhost.CaptureTest \
    dev.nativetheme.androidhost.test/androidx.test.runner.AndroidJUnitRunner >"$dir/instrument.txt" 2>&1
  "$adb" logcat -d >"$dir/logcat.txt" 2>&1
  if grep -q '^OK (50 tests)' "$dir/instrument.txt"; then
    note "run $run: CaptureTest OK"
  else
    note "run $run: CaptureTest failed"
    tail -n 40 "$dir/instrument.txt"
    failed=1
  fi
  "$adb" exec-out run-as dev.nativetheme.androidhost tar -cf - -C files capture | tar -xf - -C "$dir"
  note "run $run: $(find "$dir/capture" -name '*.png' | wc -l) PNGs"
  (cd "$dir/capture" && cat device.toml $(find . -name '*.toml' ! -name device.toml ! -name index.toml | sort) \
    >index.toml && find . -name '*.toml' ! -name index.toml -delete)
  "$adb" emu kill >/dev/null 2>&1
  for _ in $(seq 60); do kill -0 "$emulator_pid" 2>/dev/null || break; sleep 1; done
  kill -9 "$emulator_pid" 2>/dev/null
  wait "$emulator_pid" 2>/dev/null
done

pngs=$(find "$out/run1/capture" -name '*.png' | wc -l)
[ "$pngs" -eq 50 ] || { note "run 1 wrote $pngs PNGs, not 50"; failed=1; }
manifest "$out/run1/capture" >"$out/run1.sha256"
manifest "$out/run2/capture" >"$out/run2.sha256"
if [ -s "$out/run1.sha256" ] && [ "$(cat "$out/run1.sha256")" = "$(cat "$out/run2.sha256")" ]; then
  note "run 1 and run 2 byte-identical ($(wc -l <"$out/run1.sha256") files)"
else
  note "run 1 and run 2 differ:"
  diff "$out/run1.sha256" "$out/run2.sha256" | tee -a "$log"
  failed=1
fi
(cd "$out/run1/capture" && find . -name '*.png' -print | sort | xargs sha256sum) >"$out/sha256.txt"

prop() { grep -m1 "^\[$1\]" "$out/run1/getprop.txt" | sed 's/^[^:]*: \[\(.*\)\]\r*$/\1/'; }
sdk_line() { grep -m1 "^$1	" "$out/sdk.txt"; }
field() { echo "$1" | cut -f"$2"; }
{
  image_line=$(sdk_line "$image_path")
  emulator_line=$(sdk_line emulator)
  echo "[system_image]"
  echo "package = \"$image_path\""
  echo "revision = $image_revision"
  echo "archive = \"$repository/$image_archive\""
  echo "sha1 = \"$image_sha1\""
  echo "sha256 = \"$(field "$image_line" 5 | cut -d: -f2)\""
  echo "fingerprint = \"$(prop ro.build.fingerprint)\""
  echo "sdk = \"$(prop ro.build.version.sdk)\""
  echo "sdk_full = \"$(prop ro.build.version.sdk_full)\""
  echo "release = \"$(prop ro.build.version.release)\""
  echo "page_size = \"$(head -n 1 "$out/run1/display.txt" | tr -d '\r')\""
  echo
  echo "[emulator]"
  echo "revision = \"$(field "$emulator_line" 2)\""
  echo "archive = \"$repository/$(field "$emulator_line" 3)\""
  echo "sha256 = \"$(field "$emulator_line" 5 | cut -d: -f2)\""
  echo "version = \"$(grep -m1 -o 'Android emulator version .*' "$out/run1/emulator.txt")\""
  echo "gpu = \"swiftshader_indirect\""
  echo "avd = \"720x1280, 320 dpi\""
  echo
  echo "[sdk]"
  while IFS='	' read -r path revision archive sha1 sha256; do
    echo "\"$path\" = { revision = \"$revision\", archive = \"$archive\", ${sha256/:/ = \"}\" }"
  done <"$out/sdk.txt"
  echo
  echo "[compose]"
  grep -E -- '--- androidx\.compose[.:]' "$out/dependencies.txt" | grep -v ' (c)$' | sed -E 's/^[-| +\\]*//' \
    | awk '{ split($1, c, ":"); v = c[3]; for (i = 2; i < NF; i++) if ($i == "->") v = $(i + 1)
             print "\"" c[1] ":" c[2] "\" = \"" v "\"" }' | sort -u
  echo
  echo "[build]"
  echo "gradle = \"$(grep -m1 '^Gradle ' "$out/gradle-version.txt" | cut -d' ' -f2)\""
  for module in com.android.tools.build:gradle org.jetbrains.kotlin:kotlin-gradle-plugin; do
    echo "\"$module\" = \"$(grep -oE "$module:[^ ]+( -> [^ ]+)?" "$out/build-environment.txt" | head -n 1 \
      | sed -E 's/.* -> //; s/^[^:]+:[^:]+://')\""
  done
  echo "java = \"$(java -version 2>&1 | head -n 1 | tr -d '"')\""
  echo
  echo "[runner]"
  echo "image = \"${ImageOS:-?} ${ImageVersion:-?}\""
  echo "cpu = \"$(grep -m1 '^model name' /proc/cpuinfo | cut -d: -f2- | sed 's/^ //')\""
} >"$out/index.toml"
cat "$out/index.toml"
chmod -R a+rX "$out"
exit $failed
