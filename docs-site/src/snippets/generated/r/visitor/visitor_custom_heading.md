---
id: fixture_r_visitor_custom_heading
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_heading = function(ctx, level, text, id) {
    list(Custom = paste0("--- ", text, " ---"))
  }
)

result <- convert(html = "<h2>Section Title</h2><p>Content below heading.</p>", options = list(visitor = visitor))

```
