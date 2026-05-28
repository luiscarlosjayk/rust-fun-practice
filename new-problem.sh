#!/usr/bin/env bash
set -euo pipefail

# Usage: ./new-problem.sh <level> <topic> <NNNN-slug> <fn_name>
# Example: ./new-problem.sh easy arrays-hashing 0026-remove-duplicates remove_duplicates

if [ "$#" -ne 4 ]; then
  echo "Usage: $0 <level> <topic> <NNNN-slug> <fn_name>" >&2
  echo "Example: $0 easy arrays-hashing 0026-remove-duplicates remove_duplicates" >&2
  exit 1
fi

LEVEL="$1"; TOPIC="$2"; DIRNAME="$3"; FN="$4"
SLUG="${DIRNAME#*-}"          # strip leading NNNN-
PKG="$SLUG"
LIB="${PKG//-/_}"             # crate name uses underscores
DIR="$LEVEL/$TOPIC/$DIRNAME"
ROOT="$(cd "$(dirname "$0")" && pwd)"
TARGET="$ROOT/$DIR"

if [ -e "$TARGET" ]; then
  echo "Already exists: $DIR" >&2
  exit 1
fi

mkdir -p "$TARGET/src" "$TARGET/tests"

cat > "$TARGET/Cargo.toml" <<EOF
[package]
name = "$PKG"
version = "0.1.0"
edition = "2024"

[lib]
path = "src/lib.rs"
EOF

cat > "$TARGET/src/lib.rs" <<EOF
//! See README.md for the problem statement.

pub fn $FN() {
    todo!("implement $FN")
}
EOF

cat > "$TARGET/tests/tests.rs" <<EOF
// Validating tests for $PKG. Fill these in from the problem examples.
use ${LIB}::$FN;

#[test]
fn example_1() {
    todo!("add assertions");
}
EOF

cp "$ROOT/TEMPLATE.md" "$TARGET/README.md"

# Insert the new member into the workspace Cargo.toml, just above the marker.
awk -v line="    \"$DIR\"," '/NEW_PROBLEM_MARKER/{print line} {print}' \
  "$ROOT/Cargo.toml" > "$ROOT/Cargo.toml.tmp" && mv "$ROOT/Cargo.toml.tmp" "$ROOT/Cargo.toml"

echo "Created $DIR and added it to the workspace."
echo "Next: fill in README.md, src/lib.rs ($FN signature + body), and tests/tests.rs."
