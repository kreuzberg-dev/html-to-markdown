---
id: fixture_r_visitor_figure_custom_wrap
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_figure_end = function(ctx, output) {
    list(Custom = paste0(output, "\n[/FIGURE]\n"))
  },
  visit_figure_start = function(ctx) {
    list(Custom = "\n[FIGURE]\n")
  }
)

result <- convert(html = "<section><h2>Gallery</h2><figure><img src=\"photo1.jpg\" alt=\"Photo\"><figcaption>Beautiful sunset</figcaption></figure></section>", options = list(visitor = visitor))

```
