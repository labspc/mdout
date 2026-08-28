const tocLinks = [...document.querySelectorAll(".toc a")]
const headings = tocLinks
  .map((link) => document.getElementById(new URL(link.href).hash.slice(1)))
  .filter(Boolean)

if (tocLinks.length && headings.length && "IntersectionObserver" in window) {
  const setCurrent = (id) => {
    tocLinks.forEach((link) => {
      if (new URL(link.href).hash === `#${id}`) link.setAttribute("aria-current", "location")
      else link.removeAttribute("aria-current")
    })
  }
  const visible = new Map()
  const observer = new IntersectionObserver((entries) => {
    entries.forEach((entry) => visible.set(entry.target.id, entry.isIntersecting))
    const current = headings.find((heading) => visible.get(heading.id))
    if (current) setCurrent(current.id)
  }, { rootMargin: "-12% 0px -72%", threshold: [0, 1] })
  headings.forEach((heading) => observer.observe(heading))
  setCurrent(headings[0].id)
}
