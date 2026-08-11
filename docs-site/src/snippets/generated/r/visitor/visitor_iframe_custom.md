---
id: fixture_r_visitor_iframe_custom
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_iframe = function(ctx, src) {
    list(Custom = "[EMBEDDED: https://maps.example.com/embed]")
  }
)

result <- convert(html = "<p>Embedded map:</p><iframe src=\"https://maps.example.com/embed\" width=\"400\" height=\"300\"></iframe><p>End of map</p>", options = list(visitor = visitor))

```
