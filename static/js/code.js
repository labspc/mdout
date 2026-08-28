const copyIcon = '<svg aria-hidden="true" viewBox="0 0 24 24"><rect x="9" y="9" width="11" height="11" rx="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg>'
const copiedIcon = '<svg aria-hidden="true" viewBox="0 0 24 24"><path d="m5 12 4 4L19 6"></path></svg>'

async function writeClipboard(value) {
  if (navigator.clipboard?.writeText) return navigator.clipboard.writeText(value)
  const textarea = document.createElement("textarea")
  textarea.value = value
  textarea.style.position = "fixed"
  textarea.style.opacity = "0"
  document.body.append(textarea)
  textarea.select()
  document.execCommand("copy")
  textarea.remove()
}

document.querySelectorAll(".article-body pre").forEach((block) => {
  const article = block.closest(".article-body")
  const code = block.querySelector("code")
  if (!code || code.dataset.lang === "mermaid" || code.classList.contains("language-mermaid")) return

  const source = code.textContent.replace(/\n\n/g, "\n").replace(/\n$/, "")
  const labelText = code.dataset.name || code.dataset.lang
  block.classList.add("code-block-enhanced")

  if (labelText) {
    const label = document.createElement("span")
    label.className = "code-label"
    label.textContent = labelText
    block.prepend(label)
  }

  const button = document.createElement("button")
  button.className = "code-copy"
  button.type = "button"
  button.title = article?.dataset.copyLabel || "Copy code"
  button.setAttribute("aria-label", button.title)
  button.innerHTML = copyIcon
  button.addEventListener("click", async () => {
    try {
      await writeClipboard(source)
      button.innerHTML = copiedIcon
      button.title = article?.dataset.copiedLabel || "Code copied"
      button.setAttribute("aria-label", button.title)
      window.setTimeout(() => {
        button.innerHTML = copyIcon
        button.title = article?.dataset.copyLabel || "Copy code"
        button.setAttribute("aria-label", button.title)
      }, 1800)
    } catch (error) {
      console.error("Failed to copy code", error)
    }
  })
  block.prepend(button)
})
