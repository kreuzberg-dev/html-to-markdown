---
id: fixture_r_visitor_heading_bare_string_preserves_case
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
    paste0("## ", text, " ##")
  }
)

result <- convert(html = "<h2>Important Section Title</h2><p>Body.</p>", options = list(visitor = visitor))

```
