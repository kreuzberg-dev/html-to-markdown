---
id: fixture_r_visitor_skip_strong
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_strong = function(ctx, text) {
    "Skip"
  }
)

result <- convert(html = "<p>Normal <strong>bold text</strong> normal</p>", options = list(visitor = visitor))

```
