---
id: fixture_r_visitor_custom_output
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
    list(Custom = "## REPLACED HEADING")
  }
)

result <- convert(html = "<h1>Original Heading</h1>", options = list(visitor = visitor))

```
