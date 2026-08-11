---
id: fixture_r_visitor_line_break_custom
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
    list(Custom = " | ")
  }
)

result <- convert(html = "<p>First line<br>Second line<br>Third line</p>", options = list(visitor = visitor))

```
