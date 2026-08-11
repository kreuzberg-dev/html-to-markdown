---
id: fixture_r_visitor_custom_emphasis
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_emphasis = function(ctx, text) {
    list(Custom = paste0(">>>", text, "<<<"))
  }
)

result <- convert(html = "<p>This is <em>important</em> text.</p>", options = list(visitor = visitor))

```
