# labspc

`https://www.labspc.com/` 的 Markdown 博客源码，由 [mdout 0.2.0](https://crates.io/crates/mdout) 和 Zola 0.23.4 构建。

## 写作

文章保存在 `content/posts/`：

```sh
mdout check
mdout serve
```

`mdout serve` 会包含草稿并在 `http://127.0.0.1:1111/` 启动本地预览。发布前执行：

```sh
mdout build
```

推送到 `blog` 分支后，GitHub Actions 会检查内容并部署到 GitHub Pages。

## English

Markdown source for `https://www.labspc.com/`, built with [mdout 0.2.0](https://crates.io/crates/mdout) and Zola 0.23.4.

Articles live in `content/posts/`. Use `mdout serve` for local preview and push to `blog` to deploy through GitHub Pages.
