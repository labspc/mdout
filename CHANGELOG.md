# Changelog

All notable changes to mdout are documented in this file.

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
