---
id: fixture_r_visitor_horizontal_rule_custom
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_horizontal_rule = function(ctx) {
    list(Custom = "\n[DIVIDER]\n")
  }
)

result <- convert(html = "<h1>Section A</h1><p>Content A</p><hr><h1>Section B</h1><p>Content B</p>", options = list(visitor = visitor))

```
