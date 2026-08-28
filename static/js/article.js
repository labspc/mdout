async function enhanceArticle() {
  const article = document.querySelector(".article-body")
  if (!article) return

  const loadScript = (src) => new Promise((resolve, reject) => {
    const script = document.createElement("script")
    script.src = src
    script.onload = resolve
    script.onerror = reject
    document.head.append(script)
  })

  if ((article.textContent || "").includes("$") && window.renderMathInElement) {
    window.renderMathInElement(article, {
      delimiters: [
        { left: "$$", right: "$$", display: true },
        { left: "$", right: "$", display: false },
      ],
      throwOnError: false,
    })
  }

  const codeBlocks = [...new Set(article.querySelectorAll(
    'pre[data-lang="mermaid"] code, pre code[data-lang="mermaid"], pre code.language-mermaid',
  ))]
  if (!codeBlocks.length) return

  const diagrams = codeBlocks.map((code) => {
    const diagram = document.createElement("div")
    diagram.className = "mermaid"
    diagram.dataset.source = code.textContent
    code.closest("pre").replaceWith(diagram)
    return diagram
  })

  await loadScript(article.dataset.mermaid)

  async function renderDiagrams(theme) {
    for (const diagram of diagrams) diagram.textContent = diagram.dataset.source
    window.mermaid.initialize({
      startOnLoad: false,
      securityLevel: "strict",
      theme: theme === "dark" ? "dark" : "neutral",
    })
    await window.mermaid.run({ nodes: diagrams })
  }

  await renderDiagrams(document.documentElement.dataset.theme)
  document.addEventListener("themechange", (event) => renderDiagrams(event.detail.theme))
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", enhanceArticle, { once: true })
} else {
  enhanceArticle()
}
