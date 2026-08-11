---
id: fixture_r_visitor_button_skip
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_button = function(ctx, text) {
    "Skip"
  }
)

result <- convert(html = "<p>Actions available: <button>Save</button> <button>Delete</button> <button>Export</button></p>", options = list(visitor = visitor))

```
