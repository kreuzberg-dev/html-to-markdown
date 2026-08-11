---
id: fixture_r_visitor_horizontal_rule_skip
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
    "Skip"
  }
)

result <- convert(html = "<p>Part 1</p><hr><p>Part 2</p><hr><p>Part 3</p>", options = list(visitor = visitor))

```
