#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
fixture_root="$(mktemp -d "${TMPDIR:-/tmp}/mdout-fixture.XXXXXX")"
trap 'rm -rf "$fixture_root"' EXIT

require_file() {
  if [ ! -f "$1" ]; then
    echo "Missing fixture output: $1" >&2
    exit 1
  fi
}

require_text() {
  local expected="$1"
  local file="$2"
  if ! grep -F -q "$expected" "$file"; then
    echo "Fixture output does not contain '$expected': $file" >&2
    exit 1
  fi
}

mkdir -p "$fixture_root/content/posts"
cp -R "$repo_root/templates" "$repo_root/sass" "$repo_root/static" "$repo_root/reports" "$fixture_root/"
cp "$repo_root/zola.toml" "$fixture_root/zola.toml"
cp -R "$repo_root/content/." "$fixture_root/content/"
cp "$repo_root/tests/fixtures/visual/content/posts/"*.md "$fixture_root/content/posts/"

cargo run --quiet --locked --manifest-path "$repo_root/Cargo.toml" -- \
  check --directory "$fixture_root/content"

zola_bin="${MDOUT_ZOLA:-zola}"
"$zola_bin" --root "$fixture_root" check
"$zola_bin" --root "$fixture_root" build --base-url "https://example.com/mdout/"

zh_article="$fixture_root/public/posts/reading-sample/index.html"
en_article="$fixture_root/public/en/posts/reading-sample/index.html"

for output in \
  "$zh_article" \
  "$en_article" \
  "$fixture_root/public/404.html" \
  "$fixture_root/public/search_index.zh.json" \
  "$fixture_root/public/search_index.en.json" \
  "$fixture_root/public/llms.txt" \
  "$fixture_root/public/en/llms.txt" \
  "$fixture_root/public/index.xml" \
  "$fixture_root/public/en/index.xml"
do
  require_file "$output"
done

require_text 'js/code.js' "$zh_article"
require_text 'data-lang=mermaid' "$zh_article"
require_text 'katex.min.js' "$zh_article"
require_text 'Markdown in, HTML out. Built on Zola.' "$fixture_root/public/llms.txt"
require_text 'href=https://example.com/mdout/en/posts/reading-sample/ hreflang=en lang=en' "$zh_article"
require_text 'href=https://example.com/mdout/posts/reading-sample/ hreflang=zh lang=zh' "$en_article"
require_text 'href=https://example.com/mdout/en hreflang=en lang=en' "$fixture_root/public/posts/older-sample/index.html"
require_text 'An article for checking the reading experience' "$fixture_root/public/search_index.en.json"
require_text '一篇用于检查阅读体验的文章' "$fixture_root/public/search_index.zh.json"
require_text 'An article for checking the reading experience' "$fixture_root/public/en/index.xml"
require_text 'https://example.com/mdout/en/posts/reading-sample/' "$fixture_root/public/en/llms.txt"
if grep -R -E -q '(href|src)=/' "$fixture_root/public" --include='*.html'; then
  echo "Root-relative asset or navigation URL found in fixture output" >&2
  exit 1
fi

echo "Fixture verification passed"
