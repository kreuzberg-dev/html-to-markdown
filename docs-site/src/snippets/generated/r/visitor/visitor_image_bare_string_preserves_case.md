---
id: fixture_r_visitor_image_bare_string_preserves_case
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_image = function(ctx, src, alt, title) {
    paste0("[image: ", alt, " -> ", src, "]")
  }
)

result <- convert(html = "<img src=\"PhotoOne.JPG\" alt=\"Sunset Over Bay\">", options = list(visitor = visitor))

```
