#!/bin/bash
# Runs on a macOS runner (os-probe.yml's ios-host job), from the repository's
# root, with the image's default Xcode: builds tools/ios-host for the
# simulator, creates and boots a simulator of the image's newest iOS runtime
# of the SDK's major version (macos-15 also carries iOS 26 runtimes, which
# would run an iOS 18 SDK build in iOS 26's compatibility look) on the newest
# "iPhone <N> Pro" that runtime supports, runs the app twice, and
# checks that the two runs wrote byte-identical files (shasum -a 256). For each
# page the app draws, the script takes the simulator's screenshot
# (`simctl io screenshot`) when the app writes sync/ready-<page> and hands it
# over as sync/screen-<page>.png (tools/ios-host/IosHost/IosHost.swift). Writes
# into $1:
#   run<N>/       the app's draw/, screen/ and pages/ PNGs, index.toml,
#                 sync/done (ok or the app's failures)
#   environment.txt (the OS, Xcode and its SDKs, the runtime and device chosen,
#   the SDK's UIButton.Configuration styles and sizes), build.log, run.txt
#   (each step's exit status), run<N>.sha256, sha256.txt
set -u
out=$1
mkdir -p "$out"
out=$(cd "$out" && pwd)
log="$out/run.txt"
: >"$log"
note() { echo "$*" | tee -a "$log"; }
manifest() { (cd "$1" && find . -type f ! -name '*.log' -print | LC_ALL=C sort | xargs shasum -a 256); }
bundle=io.github.tiborgats.native-theme.ios-host
build="${RUNNER_TEMP:-/tmp}/ios-host-build"

sdk=$(xcrun --sdk iphonesimulator --show-sdk-path)
header="$sdk/System/Library/Frameworks/UIKit.framework/Headers/UIButtonConfiguration.h"
{
  sw_vers; echo "sw_vers: exit $?"
  xcode-select -p; echo "xcode-select -p: exit $?"
  xcodebuild -version; echo "xcodebuild -version: exit $?"
  echo "iphonesimulator SDK: $(xcrun --sdk iphonesimulator --show-sdk-version) build $(xcrun --sdk iphonesimulator --show-sdk-build-version)"
  echo "sysctl hw.model: $(sysctl -n hw.model)"
  echo "UIButtonConfiguration.h, the styles and sizes:"
  grep -n -E '\(instancetype\)[A-Za-z]*ButtonConfiguration|UIButtonConfigurationSize[A-Za-z]* *[=,]' "$header"; echo "grep: exit $?"
  echo "the iOS runtimes:"
  xcrun simctl list runtimes ios; echo "simctl list runtimes: exit $?"
} >"$out/environment.txt" 2>&1
cat "$out/environment.txt"

xcrun simctl list -j runtimes >"$out/runtimes.json"
sdk_version=$(xcrun --sdk iphonesimulator --show-sdk-version)
choice=$(python3 -I - "$out/runtimes.json" "$sdk_version" <<'EOF'
import json, re, sys
major = sys.argv[2].split(".")[0]
runtimes = [r for r in json.load(open(sys.argv[1]))["runtimes"]
            if r.get("platform") == "iOS" and r.get("isAvailable") and r["version"].split(".")[0] == major]
runtime = max(runtimes, key=lambda r: tuple(int(p) for p in r["version"].split(".")))
pro = [(int(m.group(1)), t) for t in runtime.get("supportedDeviceTypes", [])
       if (m := re.fullmatch(r"iPhone (\d+) Pro", t["name"]))]
device = max(pro, key=lambda p: p[0])[1]
print("\t".join([runtime["identifier"], runtime["version"], runtime["buildversion"],
                 device["identifier"], device["name"]]))
EOF
)
status=$?
note "choose the runtime and device: exit $status"
if [ $status -ne 0 ] || [ -z "$choice" ]; then exit 1; fi
IFS=$'\t' read -r runtime runtime_version runtime_build devicetype device_name <<<"$choice"
note "runtime: $runtime ($runtime_version, $runtime_build)"
note "device: $device_name ($devicetype)"

xcodebuild -project tools/ios-host/IosHost.xcodeproj -target IosHost -configuration Release \
  -sdk iphonesimulator ARCHS=arm64 ONLY_ACTIVE_ARCH=YES CODE_SIGNING_ALLOWED=NO \
  SYMROOT="$build" OBJROOT="$build/obj" build >"$out/build.log" 2>&1
status=$?
note "xcodebuild: exit $status"
if [ $status -ne 0 ]; then tail -n 80 "$out/build.log"; exit 1; fi
app="$build/Release-iphonesimulator/IosHost.app"
plutil -convert xml1 -o "$out/Info.plist" "$app/Info.plist"
note "Info.plist DTXcodeBuild: $(plutil -extract DTXcodeBuild raw "$app/Info.plist")"

udid=$(xcrun simctl create ios-host "$devicetype" "$runtime")
status=$?
note "simctl create: exit $status, $udid"
if [ $status -ne 0 ]; then exit 1; fi
xcrun simctl bootstatus "$udid" -b >"$out/boot.log" 2>&1
note "simctl bootstatus -b: exit $?"
xcrun simctl install "$udid" "$app"
status=$?
note "simctl install: exit $status"
if [ $status -ne 0 ]; then xcrun simctl delete "$udid"; exit 1; fi
# The first screenshots after a boot can take over a minute (macos-26):
# one is taken before the runs.
shot=$(date +%s)
xcrun simctl io "$udid" screenshot --type=png "$out/warm-up.png" >/dev/null 2>&1
note "warm-up screenshot: exit $?, $(($(date +%s) - shot)) s"

failed=0
for run in 1 2; do
  xcrun simctl launch --terminate-running-process --stdout="$out/run$run-stdout.log" \
    --stderr="$out/run$run-stderr.log" "$udid" "$bundle" "run$run"
  status=$?
  note "simctl launch run $run: exit $status"
  if [ $status -ne 0 ]; then failed=1; continue; fi
  container=$(xcrun simctl get_app_container "$udid" "$bundle" data)
  sync="$container/Documents/run$run/sync"
  page=0
  start=$(date +%s)
  while [ ! -e "$sync/done" ]; do
    if [ -e "$sync/ready-$page" ]; then
      shot=$(date +%s)
      xcrun simctl io "$udid" screenshot --type=png "$sync/.screen-$page.png" >/dev/null 2>&1
      status=$?
      if [ $status -ne 0 ]; then note "screenshot $page: exit $status"; failed=1; fi
      shot=$(($(date +%s) - shot))
      if [ $shot -gt 10 ]; then note "run $run screenshot $page: $shot s"; fi
      mv "$sync/.screen-$page.png" "$sync/screen-$page.png"
      page=$((page + 1))
    elif [ $(($(date +%s) - start)) -gt 1800 ]; then
      note "run $run: no sync/done within 1800 s"
      failed=1
      break
    else
      sleep 0.05
    fi
  done
  note "run $run: $page pages, done: $(tr '\n' ' ' <"$sync/done" 2>/dev/null)"
  [ "$(cat "$sync/done" 2>/dev/null)" = "ok" ] || failed=1
  xcrun simctl terminate "$udid" "$bundle" >/dev/null 2>&1
  rm -rf "$out/run$run"
  cp -R "$container/Documents/run$run" "$out/run$run"
  note "run $run: $(find "$out/run$run/draw" -name '*.png' | wc -l | tr -d ' ') drawn, $(find "$out/run$run/screen" -name '*.png' | wc -l | tr -d ' ') cropped PNGs"
done
xcrun simctl shutdown "$udid" >/dev/null 2>&1
xcrun simctl delete "$udid"

for run in 1 2; do manifest "$out/run$run" >"$out/run$run.sha256"; done
if [ -s "$out/run1.sha256" ] && cmp -s "$out/run1.sha256" "$out/run2.sha256"; then
  note "run 1 and run 2 byte-identical ($(wc -l <"$out/run1.sha256" | tr -d ' ') files)"
else
  note "run 1 and run 2 differ: $(diff "$out/run1.sha256" "$out/run2.sha256" | grep -c '^<') files"
  diff "$out/run1.sha256" "$out/run2.sha256" | grep '^<' | head -n 40 | tee -a "$log"
  failed=1
fi
(cd "$out" && find . -name '*.png' -print | LC_ALL=C sort | xargs shasum -a 256) >"$out/sha256.txt"
exit $failed
