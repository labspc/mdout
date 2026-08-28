# Changelog

All notable changes to mdout are documented in this file.

## [0.2.0] - 2026-08-28

### Added

- `mdout init` for generating a complete standalone blog repository offline.
- Embedded templates, styles, scripts, local KaTeX/Mermaid assets, content skeleton, and reports.
- A generated-site `mdout.toml` manifest that pins the mdout and Zola versions.
- Generated-site CI and GitHub Pages workflows that install the published CLI.
- Scaffold integration tests that build the generated site at a subpath.
- crates.io package metadata and deterministic package contents.
- Prebuilt Linux, macOS, and Windows CLI archives in GitHub Releases.

### Changed

- mdout is now distributed as a scaffold CLI as well as a full source package.
- `doctor` validates the site manifest before invoking Zola.

## [0.1.0] - 2026-08-28

### Added

- Rust CLI with `doctor`, `check`, `serve`, `build`, and `links` commands.
- A bilingual Chinese and English Zola site with optional article translations.
- Responsive reading layout, dark mode, archives, tags, RSS, and `llms.txt`.
- Browser search using Zola's per-language JSON indexes.
- Local KaTeX and Mermaid rendering without CDN dependencies.
- Syntax highlighting enhancements, filename labels, and code copying.
- External-link reports and a static link-status page.
- Visual fixtures covering prose, formulas, diagrams, code, and multilingual routing.
- CI, GitHub Pages deployment, custom-domain support, and release packaging.

### Constraints

- Articles are Markdown files with YAML Frontmatter.
- Article images and cover-image fields are intentionally unsupported.
- Zola 0.23.4 is required.
