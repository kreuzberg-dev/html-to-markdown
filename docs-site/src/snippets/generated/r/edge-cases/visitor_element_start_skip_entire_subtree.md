---
id: fixture_r_visitor_element_start_skip_entire_subtree
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_element_start = function(ctx) {
    "Skip"
  }
)

result <- convert(html = "<div><h1>Title</h1><p>Content</p></div>", options = list(visitor = visitor))

```
