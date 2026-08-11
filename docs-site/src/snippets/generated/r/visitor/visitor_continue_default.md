---
id: fixture_r_visitor_continue_default
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
    "Continue"
  }
)

result <- convert(html = "<p>Hello <strong>World</strong></p>", options = list(visitor = visitor))

```
