---
id: fixture_r_visitor_iframe_skip
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
    "Skip"
  }
)

result <- convert(html = "<h3>Reviews</h3><iframe src=\"https://widget.example.com/reviews\"></iframe><p>See reviews from our partners.</p>", options = list(visitor = visitor))

```
