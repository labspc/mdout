const themePreference = window.matchMedia("(prefers-color-scheme: dark)")

function savedTheme() {
  try {
    return localStorage.getItem("mdout-theme")
  } catch {
    return null
  }
}

function applyTheme(theme) {
  document.documentElement.dataset.theme = theme
  document.dispatchEvent(new CustomEvent("themechange", { detail: { theme } }))
}

applyTheme(savedTheme() || (themePreference.matches ? "dark" : "light"))

document.addEventListener("DOMContentLoaded", () => {
  const toggle = document.querySelector(".theme-toggle")
  toggle?.addEventListener("click", () => {
    const next = document.documentElement.dataset.theme === "dark" ? "light" : "dark"
    applyTheme(next)
    try {
      localStorage.setItem("mdout-theme", next)
    } catch {}
  })

  const currentPath = location.pathname.replace(/\/+$/, "/")
  document.querySelectorAll(".site-navigation a").forEach((link) => {
    const linkPath = new URL(link.href).pathname.replace(/\/+$/, "/")
    const exact = linkPath === currentPath
    const section = linkPath !== "/" && ["/posts/", "/tags/"].some((suffix) => linkPath.endsWith(suffix))
    if (exact || (section && currentPath.startsWith(linkPath))) {
      link.setAttribute("aria-current", exact ? "page" : "location")
    }
  })
})

themePreference.addEventListener("change", (event) => {
  if (!savedTheme()) applyTheme(event.matches ? "dark" : "light")
})
