---
id: fixture_r_visitor_underline_skip
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_underline = function(ctx, text) {
    "Skip"
  }
)

result <- convert(html = "<p>Normal text with <u>underlined part</u> and more text.</p>", options = list(visitor = visitor))

```
