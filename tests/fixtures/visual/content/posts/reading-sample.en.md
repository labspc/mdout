---
title: An article for checking the reading experience
description: Non-production content covering prose, formulas, code, tables, callouts, tags, and the table of contents.
date: 2026-08-28
taxonomies:
  tags: [writing, markdown, rust]
---

This article exists only for visual regression. Inline math uses $E = mc^2$, followed by a display formula:

$$
\int_{-\infty}^{\infty} e^{-x^2}\\,dx = \sqrt{\pi}
$$

## Prose hierarchy

English prose and `inline code` should maintain a stable reading rhythm.

> A quotation should remain clear without overpowering the article.

### Code and table

```rust,name=main.rs
fn main() {
    println!("Markdown in, HTML out. Built on Zola.");
}
```

> [!NOTE]
> Zola produces the GFM callout and mdout provides its reading style.

| Capability | Status |
| --- | --- |
| Markdown | Available |
| KaTeX | Available |
| Mermaid | Available |

## Mermaid

```mermaid
flowchart LR
    Markdown --> mdout --> HTML
```
