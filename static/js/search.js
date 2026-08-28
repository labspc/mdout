document.addEventListener("DOMContentLoaded", () => {
  const dialog = document.querySelector(".search-dialog")
  const button = document.querySelector(".search-open")
  const closeButton = dialog?.querySelector(".search-close")
  const input = dialog?.querySelector(".search-input")
  const results = dialog?.querySelector(".search-results")
  const status = dialog?.querySelector(".search-status")
  const tagData = document.querySelector(".search-tags")
  if (!dialog || !button || !closeButton || !input || !results || !status) return

  let entries
  let activeIndex = -1
  const strings = {
    loading: dialog.dataset.loading,
    prompt: dialog.dataset.prompt,
    results: dialog.dataset.results,
    empty: dialog.dataset.empty,
    error: dialog.dataset.error,
  }
  const locale = dialog.dataset.locale || document.documentElement.lang || "en"

  const normalize = (value = "") => value.normalize("NFKC").toLocaleLowerCase(locale)

  async function ensureIndex() {
    if (entries) return entries
    status.textContent = strings.loading
    const response = await fetch(dialog.dataset.index)
    if (!response.ok) throw new Error(`search index returned ${response.status}`)
    entries = await response.json()
    const tagsByPath = new Map(JSON.parse(tagData?.textContent || "[]").map((item) => [item.path, item.tags]))
    entries = entries.map((entry) => ({ ...entry, tags: tagsByPath.get(entry.path) || [] }))
    status.textContent = strings.prompt
    return entries
  }

  function excerpt(entry, query) {
    const content = (entry.body || entry.description || "").replace(/\s+/g, " ").trim()
    const tags = (entry.tags || []).map((tag) => `#${tag}`).join(" ")
    if (tags && normalize(tags).includes(normalize(query)) && !normalize(content).includes(normalize(query))) {
      return tags
    }
    if (!content) return entry.description || entry.path || ""
    const at = normalize(content).indexOf(normalize(query))
    const start = at < 0 ? 0 : Math.max(0, at - 42)
    const end = Math.min(content.length, start + 132)
    return `${start > 0 ? "…" : ""}${content.slice(start, end)}${end < content.length ? "…" : ""}`
  }

  function appendHighlighted(parent, value, query) {
    const normalizedValue = normalize(value)
    const normalizedQuery = normalize(query)
    const at = normalizedValue.indexOf(normalizedQuery)
    if (at < 0 || !normalizedQuery) {
      parent.textContent = value
      return
    }
    parent.append(document.createTextNode(value.slice(0, at)))
    const mark = document.createElement("mark")
    mark.textContent = value.slice(at, at + query.length)
    parent.append(mark, document.createTextNode(value.slice(at + query.length)))
  }

  function rank(entry, terms) {
    const title = normalize(entry.title)
    const description = normalize(entry.description)
    const body = normalize(entry.body)
    const tags = normalize((entry.tags || []).join(" "))
    let score = 0
    for (const term of terms) {
      if (title === term) score += 16
      else if (title.startsWith(term)) score += 10
      else if (title.includes(term)) score += 7
      if (description.includes(term)) score += 3
      if (tags.includes(term)) score += 5
      if (body.includes(term)) score += 1
      if (!title.includes(term) && !description.includes(term) && !tags.includes(term) && !body.includes(term)) return 0
    }
    return score
  }

  function setActive(next) {
    const links = [...results.querySelectorAll("a")]
    if (!links.length) return
    activeIndex = (next + links.length) % links.length
    links.forEach((link, index) => link.setAttribute("aria-selected", String(index === activeIndex)))
    links[activeIndex].focus()
  }

  function render() {
    const query = input.value.trim()
    results.replaceChildren()
    activeIndex = -1
    if (!query || !entries) {
      status.textContent = strings.prompt
      return
    }
    const terms = normalize(query).split(/\s+/).filter(Boolean)
    const matches = entries
      .map((entry) => ({ entry, score: rank(entry, terms) }))
      .filter(({ score }) => score > 0)
      .sort((a, b) => b.score - a.score || a.entry.title.localeCompare(b.entry.title, locale))
      .slice(0, 8)

    status.textContent = matches.length ? `${matches.length} ${strings.results}` : strings.empty
    for (const { entry } of matches) {
      const link = document.createElement("a")
      link.className = "search-result"
      link.href = entry.url || entry.path
      link.setAttribute("role", "option")
      link.setAttribute("aria-selected", "false")
      const title = document.createElement("strong")
      appendHighlighted(title, entry.title, query)
      const summary = document.createElement("span")
      appendHighlighted(summary, excerpt(entry, query), query)
      link.append(title, summary)
      results.append(link)
    }
  }

  async function open() {
    dialog.hidden = false
    document.body.classList.add("search-is-open")
    input.focus()
    try {
      await ensureIndex()
      render()
    } catch (error) {
      status.textContent = strings.error
      console.error(error)
    }
  }

  function close() {
    dialog.hidden = true
    document.body.classList.remove("search-is-open")
    input.value = ""
    results.replaceChildren()
    status.textContent = strings.prompt
    activeIndex = -1
    button.focus()
  }

  button.addEventListener("click", open)
  closeButton.addEventListener("click", close)
  input.addEventListener("input", render)
  dialog.addEventListener("click", (event) => {
    if (event.target === dialog) close()
  })
  dialog.addEventListener("keydown", (event) => {
    if (event.key === "ArrowDown") {
      event.preventDefault()
      setActive(activeIndex + 1)
    } else if (event.key === "ArrowUp") {
      event.preventDefault()
      setActive(activeIndex - 1)
    } else if (event.key === "Tab") {
      const focusable = [input, ...results.querySelectorAll("a"), closeButton]
      const current = focusable.indexOf(document.activeElement)
      if (event.shiftKey && current === 0) {
        event.preventDefault()
        focusable.at(-1).focus()
      } else if (!event.shiftKey && current === focusable.length - 1) {
        event.preventDefault()
        focusable[0].focus()
      }
    }
  })
  document.addEventListener("keydown", (event) => {
    if ((event.metaKey || event.ctrlKey) && event.key.toLowerCase() === "k") {
      event.preventDefault()
      dialog.hidden ? open() : close()
    } else if (event.key === "Escape" && !dialog.hidden) {
      close()
    }
  })
})
