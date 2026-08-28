#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
fixture_root="$(mktemp -d "${TMPDIR:-/tmp}/mdout-scaffold.XXXXXX")"
trap 'rm -rf "$fixture_root"' EXIT
site="$fixture_root/blog"

cargo run --quiet --locked --manifest-path "$repo_root/Cargo.toml" -- \
  init "$site" \
  --title "Generated notes" \
  --base-url "https://example.com/notes/" \
  --author "mdout test"

test -f "$site/mdout.toml"
test -f "$site/.github/workflows/ci.yml"
test -f "$site/templates/base.html"
test -f "$site/static/js/search.js"
test ! -e "$site/Cargo.toml"
test ! -e "$site/src"

(
  cd "$site"
  cargo run --quiet --locked --manifest-path "$repo_root/Cargo.toml" -- doctor
  cargo run --quiet --locked --manifest-path "$repo_root/Cargo.toml" -- check
  cargo run --quiet --locked --manifest-path "$repo_root/Cargo.toml" -- \
    build --base-url "https://example.com/notes/"
)

test -f "$site/public/index.html"
grep -F -q '<title>Generated notes' "$site/public/index.html"
if grep -R -E -q '(href|src)=/' "$site/public" --include='*.html'; then
  echo "Root-relative URL found in generated site" >&2
  exit 1
fi

echo "Scaffold verification passed"
