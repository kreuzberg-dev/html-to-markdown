---
id: fixture_r_visitor_skip_heading
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
    "Skip"
  }
)

result <- convert(html = "<h1>Title</h1><p>Body text remains.</p>", options = list(visitor = visitor))

```
