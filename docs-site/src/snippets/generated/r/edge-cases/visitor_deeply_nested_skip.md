---
id: fixture_r_visitor_deeply_nested_skip
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_mark = function(ctx, text) {
    "Skip"
  }
)

result <- convert(html = "<div><p>Outer <em>emphasis <strong>with bold <mark>and highlight</mark></strong></em> text</p></div>", options = list(visitor = visitor))

```
