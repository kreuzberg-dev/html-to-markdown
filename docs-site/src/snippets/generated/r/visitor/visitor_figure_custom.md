---
id: fixture_r_visitor_figure_custom
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_figcaption = function(ctx, text) {
    list(Custom = paste0("*", text, "*"))
  }
)

result <- convert(html = "<article><h1>Article Title</h1><p>Introduction paragraph.</p><figure><img src=\"diagram.png\" alt=\"System architecture diagram\"><figcaption>Figure 1: System Architecture</figcaption></figure><p>Explanation of the figure.</p></article>", options = list(visitor = visitor))

```
