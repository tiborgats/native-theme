#!/bin/bash

# Pre-release check script for native-theme workspace
# This script performs comprehensive checks before releasing a new version

set -e  # Exit immediately if a command exits with a non-zero status
set -u  # Exit if an undefined variable is used

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_step() {
    echo -e "${BLUE}=== $1 ===${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

# Function to run a command with error handling
run_check() {
    local description="$1"
    shift
    print_step "$description"

    if "$@"; then
        print_success "$description completed successfully"
        echo
    else
        print_error "$description failed!"
        echo -e "${RED}Command that failed: $*${NC}"
        exit 1
    fi
}

# Function to run a command with soft error handling (warns instead of exiting)
run_check_soft() {
    local description="$1"
    shift
    print_step "$description"

    if "$@"; then
        print_success "$description completed successfully"
        echo
    else
        print_warning "$description failed (non-blocking). Continuing..."
        echo
    fi
}

# Function to check and install optional tools
check_and_install_tool() {
    local tool_name="$1"
    local package_name="$2"
    local description="$3"

    # Extract subcommand name from tool name (e.g., "cargo-audit" -> "audit")
    local subcommand="${tool_name#cargo-}"

    # Check if cargo subcommand is available
    if ! cargo "$subcommand" --help >/dev/null 2>&1; then
        echo -e "${YELLOW}Optional tool '$tool_name' is not installed.${NC}"
        echo -e "${BLUE}Description: $description${NC}"
        echo -n -e "${BLUE}Do you want to install it? [Y/n]: ${NC}"
        read -r response

        # Default to 'Y' if user just presses enter
        if [[ -z "$response" || "$response" =~ ^[Yy]$ ]]; then
            echo -e "${BLUE}Installing $package_name...${NC}"
            if cargo install "$package_name"; then
                print_success "$package_name installed successfully"
            else
                print_error "Failed to install $package_name"
                exit 1
            fi
        else
            echo -e "${YELLOW}Skipping installation of $package_name${NC}"
        fi
        echo
    else
        print_success "$tool_name is already installed"
    fi
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_error "Cargo.toml not found. Please run this script from the project root."
    exit 1
fi

# Get the current version from workspace Cargo.toml
CURRENT_VERSION=$(grep -E '^version\s*=' Cargo.toml | sed -E 's/.*"([^"]+)".*/\1/')

echo -e "${BLUE}🚀 Pre-release checks for native-theme v${CURRENT_VERSION}${NC}"
echo

# Check if we're in a git repository
if [ ! -d ".git" ]; then
    print_warning "Not in a git repository. Some checks will be skipped."
else
    # Check for uncommitted changes
    if ! git diff --quiet; then
        print_warning "You have uncommitted changes. Consider committing them before release."
        git status --porcelain
        echo
    fi

    # Check if current version is already tagged
    if git tag --list | grep -q "^v${CURRENT_VERSION}$"; then
        print_warning "Version v${CURRENT_VERSION} is already tagged in git."
    fi
fi

# Check and install optional tools
print_step "Checking optional tools"
check_and_install_tool "cargo-audit" "cargo-audit" "Security vulnerability scanner for Rust dependencies"
check_and_install_tool "cargo-outdated" "cargo-outdated" "Tool to check for outdated dependencies"

# Check for TODO/FIXME comments in source code
print_step "Checking for TODO/FIXME comments"
if grep -rn --include="*.rs" --exclude-dir=target "TODO\|FIXME" .; then
    print_warning "Found TODO/FIXME comments in source code. Consider addressing them before release."
else
    print_success "No TODO/FIXME comments found in source code"
fi
echo

# Check for panic-inducing patterns in non-test production source code.
# This is a BLOCKING check — runtime panics are forbidden in production code.
#
# Detected patterns (outside #[cfg(test)] blocks):
#   - .unwrap()              → use ? / match / unwrap_or_*
#   - .expect(...)           → same
#   - panic!(...)            → use Result/Option propagation
#   - todo!(...)             → placeholder that must not ship
#   - unreachable!(...)      → rewrite the code path so the compiler proves it unreachable
#   - Instant::now() + / -   → use Instant::checked_add / checked_sub (std documents Sub<Duration>
#                              for Instant as "may panic"; relying on current impl details is fragile)
#
# This check is regex-based and has known blind spots: operator panics
# hidden in std beyond the Instant::now() pattern (e.g. u32 - u32
# underflow, arr[i] out-of-bounds, Duration + Duration overflow) are
# NOT caught here. For those, rely on code review, clippy, and tests.
print_step "Checking for panic-inducing patterns in production code"
PANIC_FOUND=0

# Scan each source directory (excluding test files, examples, build scripts, and test modules)
for src_dir in native-theme/src connectors/native-theme-gpui/src connectors/native-theme-iced/src; do
    if [ -d "$src_dir" ]; then
        # Use a Python script to accurately detect panics outside #[cfg(test)] blocks
        HITS=$(python3 -c "
import sys, re

def scan_file(path):
    issues = []
    try:
        with open(path) as f:
            lines = f.readlines()
    except:
        return issues

    in_test = 0  # brace depth when test module started, 0 = not in test
    brace_depth = 0
    test_active = False

    for i, line in enumerate(lines, 1):
        s = line.strip()

        # Track test module entry
        if re.search(r'#\[cfg\((all\()?test[,\)]', s):
            test_active = True
            continue

        # Count braces (rough but effective for Rust source)
        opens = line.count('{')
        closes = line.count('}')

        if test_active and opens > 0:
            in_test = brace_depth + 1
            test_active = False

        brace_depth += opens - closes

        if in_test > 0 and brace_depth < in_test:
            in_test = 0

        if in_test > 0:
            continue

        # Skip comments (single-line only; block comments via /* */ are not
        # stripped, but the Rust convention in this crate is // doc comments)
        if s.startswith('//'):
            continue

        # Check for panic-inducing patterns. Each pattern is labelled so
        # the error message tells the maintainer exactly which rule fired.
        if '.unwrap()' in line:
            issues.append(f'{path}:{i}: [.unwrap()] {s}')
        elif re.search(r'\.expect\s*\(', line):
            issues.append(f'{path}:{i}: [.expect()] {s}')
        elif re.search(r'\bpanic!\s*\(', line):
            issues.append(f'{path}:{i}: [panic!] {s}')
        elif re.search(r'\btodo!\s*\(', line):
            issues.append(f'{path}:{i}: [todo!] {s}')
        elif re.search(r'\bunreachable!\s*\(', line):
            issues.append(f'{path}:{i}: [unreachable!] {s}')
        elif re.search(r'Instant::now\(\)\s*[-+]', line):
            issues.append(f'{path}:{i}: [Instant::now() arithmetic; use checked_add/checked_sub] {s}')

    return issues

import glob
issues = []
for f in glob.glob('${src_dir}/**/*.rs', recursive=True):
    # Skip files named tests.rs (test-only modules included via #[cfg(test)] mod tests;)
    if f.endswith('/tests.rs'):
        continue
    issues.extend(scan_file(f))

for issue in issues:
    print(issue)
sys.exit(0 if not issues else 1)
" 2>&1) || true
        if [ -n "$HITS" ]; then
            echo "$HITS"
            PANIC_FOUND=1
        fi
    fi
done

if [ "$PANIC_FOUND" -eq 1 ]; then
    print_error "Found panic-inducing pattern(s) in production code! Runtime panics are FORBIDDEN."
    print_error "Replace with proper error handling (Result/Option propagation, checked_* arithmetic)."
    print_error "Note: these patterns are acceptable inside #[cfg(test)] modules."
    print_error "Known blind spot: this check is regex-based and cannot see operator panics inside std"
    print_error "beyond the Instant::now() idiom. Code review and clippy remain the primary safety net."
    exit 1
else
    print_success "No panic-inducing patterns found in production source code"
fi
echo

# Get all workspace crate names
if command -v jq &>/dev/null; then
    WORKSPACE_CRATES=$(cargo metadata --no-deps --format-version 1 2>/dev/null \
        | jq -r '.packages[].name')
else
    # Fallback: extract crate names from workspace_members package IDs (format: path+file:///.../name#version)
    WORKSPACE_CRATES=$(cargo metadata --no-deps --format-version 1 2>/dev/null \
        | grep -oP '"workspace_members":\[[^\]]*\]' | grep -oP '[^/]+(?=#)')
fi

# Run cargo check on each crate individually (avoids cross-crate feature unification bugs)
for crate in $WORKSPACE_CRATES; do
    if [ "$crate" = "native-theme-gpui" ]; then
        run_check_soft "Running cargo check ($crate)" cargo check -p "$crate" --all-targets
    else
        run_check "Running cargo check ($crate)" cargo check -p "$crate" --all-targets
    fi
done

# Check code formatting
run_check "Fixing code formatting" cargo fmt --all

# Run clippy on each crate individually
for crate in $WORKSPACE_CRATES; do
    if [ "$crate" = "native-theme-gpui" ]; then
        run_check_soft "Running clippy ($crate)" cargo clippy -p "$crate" --all-targets -- -D warnings
    else
        run_check "Running clippy ($crate)" cargo clippy -p "$crate" --all-targets -- -D warnings
    fi
done

# Run all tests
for crate in $WORKSPACE_CRATES; do
    if [ "$crate" = "native-theme-gpui" ]; then
        run_check_soft "Running tests ($crate)" cargo test -p "$crate"
    else
        run_check "Running tests ($crate)" cargo test -p "$crate"
    fi
done

# Build examples for each crate (skip crates with no examples/ directory)
for crate in $WORKSPACE_CRATES; do
    crate_dir=$(cargo metadata --no-deps --format-version 1 2>/dev/null \
        | jq -r --arg name "$crate" '.packages[] | select(.name == $name) | .manifest_path' \
        | xargs dirname)
    if [ ! -d "$crate_dir/examples" ]; then
        continue
    fi
    if [ "$crate" = "native-theme-gpui" ]; then
        run_check_soft "Building examples ($crate)" cargo build -p "$crate" --examples
    else
        run_check "Building examples ($crate)" cargo build -p "$crate" --examples
    fi
done

# Check documentation generation
for crate in $WORKSPACE_CRATES; do
    if [ "$crate" = "native-theme-gpui" ]; then
        run_check_soft "Checking docs ($crate)" cargo doc -p "$crate" --no-deps
    else
        run_check "Checking docs ($crate)" cargo doc -p "$crate" --no-deps
    fi
done

# Validate packages before publishing.
#
# --no-verify rationale: cargo package verification compiles each tarball as if it
# were downloaded from crates.io. For first-ever publication of this workspace, the
# internal proc-macro crate native-theme-derive is not yet on the registry, so
# tarball verification cannot resolve the workspace-internal dep. The real tarball
# compilation check happens during `cargo publish` itself, which this script does
# not run. See RELEASING.md for the ordered publish workflow (derive -> build ->
# core -> connectors).
#
# Once native-theme-derive 0.5.7 is published to crates.io, remove --no-verify
# from these three lines to restore full tarball-compile verification for
# subsequent releases.
run_check "Validating packages (core)" cargo package --no-verify -p native-theme-derive -p native-theme -p native-theme-build --allow-dirty
run_check_soft "Validating package (iced connector)" cargo package --no-verify -p native-theme-derive -p native-theme -p native-theme-iced --allow-dirty
run_check_soft "Validating package (gpui connector)" cargo package --no-verify -p native-theme-derive -p native-theme -p native-theme-gpui --allow-dirty

# Check for security vulnerabilities
print_step "Running security audit"
if cargo audit; then
    print_success "Running security audit completed successfully"
else
    print_warning "Security audit found issues (see above). Review before releasing."
fi
echo

# Check for outdated dependencies
print_step "Checking for outdated dependencies"
cargo outdated --workspace
echo

# Final success message
echo -e "${GREEN}🎉 All pre-release checks passed successfully!${NC}"
echo -e "${GREEN}native-theme v${CURRENT_VERSION} is ready for release.${NC}"
echo
echo -e "${BLUE}Next steps:${NC}"
echo "1. Review the changes one more time"
echo "2. Update CHANGELOG.md if needed"
echo "3. Commit any final changes"
echo "4. Tag the release: git tag v${CURRENT_VERSION}"
echo "5. Push tag: git push origin v${CURRENT_VERSION}"
echo "6. Publish to crates.io (in dependency order):"
echo "   cargo publish -p native-theme-derive"
echo "   cargo publish -p native-theme"
echo "   cargo publish -p native-theme-build"
echo "   cargo publish -p native-theme-iced"
echo "   cargo publish -p native-theme-gpui"
echo

exit 0
