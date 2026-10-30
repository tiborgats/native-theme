#!/bin/bash
# Runs on a macOS runner (os-probe.yml's ios-host job), from the repository's
# root, with the image's default Xcode: builds tools/ios-host (the IosHost app
# and its IosHostUITests press pass) for the simulator, creates and boots a
# simulator of the image's newest iOS runtime of the SDK's major version
# (macos-15 also carries iOS 26 runtimes, which would run an iOS 18 SDK build
# in iOS 26's compatibility look) on the newest "iPhone <N> Pro" that runtime
# supports, runs the app's render pass twice (`simctl launch`) and its press
# pass twice (the UI test, `xcodebuild test-without-building`), checks that the
# two runs of each wrote the same measurements (shasum -a 256, then
# compare.swift for the files that differ; below), and solves the layers
# captured over black and white (solve.swift). For each page the app draws, the
# script takes the simulator's screenshot (`simctl io screenshot`) when the app
# writes sync/ready-<n>-<attempt> and hands it over as
# sync/screen-<n>-<attempt>.png (tools/ios-host/IosHost/IosHost.swift).
# IOS_HOST_QUICK=1 runs a few specimens at medium only. Writes into $1:
#   run<N>/, press<N>/  the app's draw/, screen/ and pages/ PNGs, index.toml;
#                 sync/ (the handshake, outside the comparison): done (ok or
#                 the app's failures), retakes (screenshots retaken)
#   solved/run1/, solved/press1/  the solved layers; solved/run1.toml,
#                 solved/press1.toml  their summaries
#   headers/      the UIKit (and AVRoutePickerView) headers and UIKit's Swift
#                 interfaces of the iphoneos and iphonesimulator SDKs, with
#                 README.txt: the Xcode, each SDK's path, version and build,
#                 each file's SHA-256
#   colors.txt    the UIColor class colours of the SDK's UIColor.h and
#                 UIInterface.h, which the app resolves
#   environment.txt (the OS, Xcode and its SDKs, the runtime and device chosen,
#   the SDK's UIButton.Configuration styles and sizes), build.log,
#   press<N>-test.log and press<N>.xcresult, run.txt (each step's exit
#   status), run<N>.sha256 and press<N>.sha256, differ.txt and
#   press-differ.txt (the files that differ between the runs),
#   differ-levels.txt and press-differ-levels.txt (by how much), sha256.txt
set -u
out=$1
quick=${IOS_HOST_QUICK:-1}
mkdir -p "$out"
out=$(cd "$out" && pwd)
log="$out/run.txt"
: >"$log"
note() { echo "$*" | tee -a "$log"; }
# manifest <folder> [<excluded path pattern>]
manifest() { (cd "$1" && find . -type f ! -path './sync/*' ! -path "${2:-./sync/*}" -print | LC_ALL=C sort | xargs shasum -a 256); }
bundle=io.github.tiborgats.native-theme.ios-host
build="${RUNNER_TEMP:-/tmp}/ios-host-build"
note "quick: $quick"

sdk=$(xcrun --sdk iphonesimulator --show-sdk-path)
uikit="$sdk/System/Library/Frameworks/UIKit.framework/Headers"
{
  sw_vers; echo "sw_vers: exit $?"
  xcode-select -p; echo "xcode-select -p: exit $?"
  xcodebuild -version; echo "xcodebuild -version: exit $?"
  echo "iphonesimulator SDK: $(xcrun --sdk iphonesimulator --show-sdk-version) build $(xcrun --sdk iphonesimulator --show-sdk-build-version)"
  echo "sysctl hw.model: $(sysctl -n hw.model)"
  echo "UIButtonConfiguration.h, the styles and sizes:"
  grep -n -E '\(instancetype\)[A-Za-z]*ButtonConfiguration|UIButtonConfigurationSize[A-Za-z]* *[=,]' "$uikit/UIButtonConfiguration.h"; echo "grep: exit $?"
  echo "the iOS runtimes:"
  xcrun simctl list runtimes ios; echo "simctl list runtimes: exit $?"
} >"$out/environment.txt" 2>&1
cat "$out/environment.txt"

# The headers a lane quotes, from both SDKs of the default Xcode.
headers="$out/headers"
mkdir -p "$headers"
{
  echo "xcode-select -p: $(xcode-select -p)"
  xcodebuild -version
  for kind in iphoneos iphonesimulator; do
    path=$(xcrun --sdk "$kind" --show-sdk-path)
    echo "$kind SDK: $path, version $(xcrun --sdk "$kind" --show-sdk-version), build $(xcrun --sdk "$kind" --show-sdk-build-version)"
    mkdir -p "$headers/$kind"
    for name in UIButton.h UIButtonConfiguration.h UIColor.h UIInterface.h UIFont.h UIFontDescriptor.h \
      UIFontMetrics.h UIImage.h UIImageConfiguration.h UIImageSymbolConfiguration.h UIBackgroundConfiguration.h \
      UIControl.h UITraitCollection.h UIContentSizeCategory.h UIView.h UIBarButtonItem.h UIBarItem.h \
      UINavigationItem.h UINavigationBar.h UIToolbar.h UIPasteControl.h UIScrollView.h \
      UILongPressGestureRecognizer.h UITapGestureRecognizer.h UINavigationController.h UITextInput.h \
      UIFocus.h UIFocusSystem.h UIAccessibility.h UIActivityIndicatorView.h UIMenu.h; do
      file="$path/System/Library/Frameworks/UIKit.framework/Headers/$name"
      if [ -f "$file" ]; then
        cp "$file" "$headers/$kind/$name"
        echo "  UIKit.framework/Headers/$name: sha256 $(shasum -a 256 <"$file" | cut -d' ' -f1)"
      else
        echo "  UIKit.framework/Headers/$name: absent"
      fi
    done
    file="$path/System/Library/Frameworks/AVKit.framework/Headers/AVRoutePickerView.h"
    if [ -f "$file" ]; then
      cp "$file" "$headers/$kind/AVRoutePickerView.h"
      echo "  AVKit.framework/Headers/AVRoutePickerView.h: sha256 $(shasum -a 256 <"$file" | cut -d' ' -f1)"
    else
      echo "  AVKit.framework/Headers/AVRoutePickerView.h: absent"
    fi
    found=0
    for file in "$path"/System/Library/Frameworks/UIKit.framework/Modules/UIKit.swiftmodule/arm64*.swiftinterface; do
      [ -f "$file" ] || continue
      found=1
      cp "$file" "$headers/$kind/"
      echo "  UIKit.framework/Modules/UIKit.swiftmodule/$(basename "$file"): sha256 $(shasum -a 256 <"$file" | cut -d' ' -f1)"
    done
    [ $found -eq 1 ] || echo "  UIKit.framework/Modules/UIKit.swiftmodule/arm64*.swiftinterface: absent"
  done
} >"$headers/README.txt" 2>&1
note "headers: $(find "$headers" -type f | wc -l | tr -d ' ') files"

colors=$(python3 -I - "$uikit/UIColor.h" "$uikit/UIInterface.h" <<'EOF'
import re, sys
names = []
for path in sys.argv[1:]:
    text = open(path, encoding="utf-8", errors="replace").read()
    pattern = r'@property\s*\(([^)]*)\)\s*UIColor\s*\*\s*(?:_Nonnull\s+|_Nullable\s+|nonnull\s+|nullable\s+)?(\w+)'
    for match in re.finditer(pattern, text):
        if re.search(r'\bclass\b', match.group(1)) and match.group(2) not in names:
            names.append(match.group(2))
print(",".join(names))
EOF
)
status=$?
echo "$colors" | tr ',' '\n' >"$out/colors.txt"
note "colour names from UIColor.h and UIInterface.h: exit $status, $(grep -c . "$out/colors.txt")"
if [ $status -ne 0 ] || [ -z "$colors" ]; then exit 1; fi

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

# One build for both passes: the app and the UI test, signed to run locally
# (a UI test runner the simulator launches needs a signature).
xcodebuild build-for-testing -project tools/ios-host/IosHost.xcodeproj -scheme IosHost -configuration Release \
  -destination "generic/platform=iOS Simulator" -derivedDataPath "$build" ARCHS=arm64 ONLY_ACTIVE_ARCH=NO \
  CODE_SIGN_IDENTITY=- CODE_SIGNING_REQUIRED=NO >"$out/build.log" 2>&1
status=$?
note "xcodebuild build-for-testing: exit $status"
if [ $status -ne 0 ]; then grep -E 'error:|warning: .*IosHost' "$out/build.log" | head -n 60; tail -n 40 "$out/build.log"; exit 1; fi
app="$build/Build/Products/Release-iphonesimulator/IosHost.app"
xctestrun=$(find "$build/Build/Products" -maxdepth 1 -name '*.xctestrun' | head -n 1)
note "xctestrun: $xctestrun"
if [ -z "$xctestrun" ]; then exit 1; fi
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

# serve <label> <documents folder> [<pid>]: takes the screenshots the app asks
# for until it writes sync/done (or <pid>, the UI test, ends first); sets
# $container and $sync; 1 where it failed.
serve() {
  local label=$1 folder=$2 pid=${3:-}
  local page=0 attempt=0 taken= start shot status
  start=$(date +%s)
  sync=
  while [ -z "$sync" ]; do
    container=$(xcrun simctl get_app_container "$udid" "$bundle" data 2>/dev/null)
    if [ -n "$container" ] && [ -d "$container/Documents/$folder/sync" ]; then
      sync="$container/Documents/$folder/sync"
    elif [ -n "$pid" ] && ! kill -0 "$pid" 2>/dev/null; then
      note "$label: the test ended before the app wrote $folder/sync"
      return 1
    elif [ $(($(date +%s) - start)) -gt 900 ]; then
      note "$label: no $folder/sync within 900 s"
      return 1
    else
      sleep 1
    fi
  done
  while [ ! -e "$sync/done" ]; do
    if [ "$taken" != "$page-$attempt" ] && [ -e "$sync/ready-$page-$attempt" ]; then
      shot=$(date +%s)
      xcrun simctl io "$udid" screenshot --type=png "$sync/.screen-$page-$attempt.png" >/dev/null 2>&1
      status=$?
      if [ $status -ne 0 ]; then note "$label screenshot $page-$attempt: exit $status"; return 1; fi
      shot=$(($(date +%s) - shot))
      if [ $shot -gt 10 ]; then note "$label screenshot $page-$attempt: $shot s"; fi
      mv "$sync/.screen-$page-$attempt.png" "$sync/screen-$page-$attempt.png"
      taken="$page-$attempt"
    elif [ "$taken" = "$page-$attempt" ] && [ -e "$sync/ready-$page-$((attempt + 1))" ]; then
      attempt=$((attempt + 1))
    elif [ "$taken" = "$page-$attempt" ] && [ -e "$sync/ready-$((page + 1))-0" ]; then
      page=$((page + 1))
      attempt=0
    elif [ -n "$pid" ] && ! kill -0 "$pid" 2>/dev/null; then
      note "$label: the test ended before sync/done"
      return 1
    elif [ $(($(date +%s) - start)) -gt 10800 ]; then
      note "$label: no sync/done within 10800 s"
      return 1
    else
      sleep 0.05
    fi
  done
  return 0
}

# keep <label> <folder>: copies the app's output of a pass into $out/<folder>.
keep() {
  local label=$1 folder=$2
  note "$label: $(find "$sync/../pages" -name '*.png' 2>/dev/null | wc -l | tr -d ' ') screenshots, done: $(tr '\n' ' ' <"$sync/done" 2>/dev/null)"
  if [ -s "$sync/retakes" ]; then sed "s/^/$label retake: /" "$sync/retakes" | tee -a "$log"; fi
  rm -rf "$out/$folder"
  cp -R "$container/Documents/$folder" "$out/$folder"
  note "$label: $(find "$out/$folder/draw" -name '*.png' | wc -l | tr -d ' ') drawn, $(find "$out/$folder/screen" -name '*.png' | wc -l | tr -d ' ') cropped PNGs"
  [ "$(cat "$sync/done" 2>/dev/null)" = "ok" ]
}

failed=0
quick_argument=()
[ "$quick" = "1" ] && quick_argument=(quick)
for run in 1 2; do
  xcrun simctl launch --terminate-running-process --stdout="$out/run$run-stdout.log" \
    --stderr="$out/run$run-stderr.log" "$udid" "$bundle" "run$run" "colors=$colors" ${quick_argument[@]+"${quick_argument[@]}"}
  status=$?
  note "simctl launch run $run: exit $status"
  if [ $status -ne 0 ]; then failed=1; continue; fi
  serve "run $run" "run$run" || failed=1
  xcrun simctl terminate "$udid" "$bundle" >/dev/null 2>&1
  [ -n "$sync" ] && { keep "run $run" "run$run" || failed=1; }
done

for run in 1 2; do
  TEST_RUNNER_IOS_HOST_RUN="press$run" TEST_RUNNER_IOS_HOST_QUICK="$quick" xcodebuild test-without-building \
    -xctestrun "$xctestrun" -destination "id=$udid" -resultBundlePath "$out/press$run.xcresult" \
    >"$out/press$run-test.log" 2>&1 &
  pid=$!
  serve "press $run" "press$run" "$pid" || failed=1
  wait "$pid"
  status=$?
  note "press $run xcodebuild test-without-building: exit $status"
  if [ $status -ne 0 ]; then failed=1; grep -E 'error|failed|Fail' "$out/press$run-test.log" | head -n 40; fi
  xcrun simctl terminate "$udid" "$bundle" >/dev/null 2>&1
  [ -n "$sync" ] && { keep "press $run" "press$run" || failed=1; }
done
xcrun simctl shutdown "$udid" >/dev/null 2>&1
xcrun simctl delete "$udid"

# The measurement is the screenshot (screen/, pages/, index.toml). Files that
# differ between the runs are compared pixel by pixel (compare.swift): a
# measurement file of the render pass differing by more than a level fails the
# job; iOS 26's Liquid Glass edges vary by one level in a few pixels, and
# drawHierarchy's copies (draw/) of the glass styles by up to three, which are
# listed in differ-levels.txt and fail nothing (spike I). The press pass's
# differences are listed in press-differ-levels.txt and fail nothing: its
# touches and animations are not frame-exact; its full screenshots (pages/)
# are left out, a retaken press adding one.
# compare <first> <second> <differ> <levels> [--list-only <excluded path pattern>]
compare() {
  local first=$1 second=$2 differ=$3 levels=$4 option=${5:-} excluded=${6:-}
  manifest "$out/$first" "$excluded" >"$out/$first.sha256"
  manifest "$out/$second" "$excluded" >"$out/$second.sha256"
  diff "$out/$first.sha256" "$out/$second.sha256" | sed -n 's/^< [0-9a-f]*  //p' >"$out/$differ"
  if [ ! -s "$out/$first.sha256" ] || [ "$(wc -l <"$out/$first.sha256")" -ne "$(wc -l <"$out/$second.sha256")" ]; then
    note "$first and $second wrote different file sets"
    return 1
  fi
  if [ -s "$out/$differ" ]; then
    note "$first and $second differ: $(grep -c . "$out/$differ") of $(wc -l <"$out/$first.sha256" | tr -d ' ') files, $(grep -vc '^\./draw/' "$out/$differ") outside draw/"
  else
    note "$first and $second byte-identical ($(wc -l <"$out/$first.sha256" | tr -d ' ') files)"
  fi
  xcrun swift tools/ios-host/compare.swift $option "$out/$first" "$out/$second" "$out/$differ" >"$out/$levels" 2>&1
  local status=$?
  head -n 60 "$out/$levels" | tee -a "$log"
  note "compare.swift $first $second: exit $status"
  return $status
}
if [ -d "$out/run1" ] && [ -d "$out/run2" ]; then
  compare run1 run2 differ.txt differ-levels.txt || failed=1
else
  failed=1
fi
if [ -d "$out/press1" ] && [ -d "$out/press2" ]; then
  compare press1 press2 press-differ.txt press-differ-levels.txt --list-only './pages/*' || failed=1
else
  failed=1
fi

mkdir -p "$out/solved"
for folder in run1 press1; do
  [ -d "$out/$folder" ] || continue
  xcrun swift tools/ios-host/solve.swift "$out/$folder" "$out/solved/$folder" "$out/solved/$folder.toml" 2>&1 | tee -a "$log"
  status=${PIPESTATUS[0]}
  note "solve.swift $folder: exit $status"
  [ "$status" -eq 0 ] || failed=1
done
(cd "$out" && find . -name '*.png' ! -path './solved/*' -print | LC_ALL=C sort | xargs shasum -a 256) >"$out/sha256.txt"
exit $failed
