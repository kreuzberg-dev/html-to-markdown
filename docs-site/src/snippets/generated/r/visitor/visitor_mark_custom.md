---
id: fixture_r_visitor_mark_custom
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_mark = function(ctx, text) {
    list(Custom = paste0("==", text, "=="))
  }
)

result <- convert(html = "<p>This is a <mark>highlighted passage</mark> in the text.</p>", options = list(visitor = visitor))

```
