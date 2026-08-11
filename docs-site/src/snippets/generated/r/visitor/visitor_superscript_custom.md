---
id: fixture_r_visitor_superscript_custom
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_superscript = function(ctx, text) {
    list(Custom = paste0("^", text, "^"))
  }
)

result <- convert(html = "<p>Einstein's E=mc<sup>2</sup> revolutionized physics.</p>", options = list(visitor = visitor))

```
