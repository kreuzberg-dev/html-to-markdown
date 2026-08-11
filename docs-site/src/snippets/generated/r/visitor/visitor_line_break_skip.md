---
id: fixture_r_visitor_line_break_skip
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_line_break = function(ctx) {
    "Skip"
  }
)

result <- convert(html = "<p>Address Line 1<br>Address Line 2<br>Address Line 3</p>", options = list(visitor = visitor))

```
