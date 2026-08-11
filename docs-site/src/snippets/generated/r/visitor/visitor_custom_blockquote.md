---
id: fixture_r_visitor_custom_blockquote
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_blockquote = function(ctx, content, depth) {
    list(Custom = paste0("QUOTE: \"", content, "\""))
  }
)

result <- convert(html = "<blockquote><p>A wise quote.</p></blockquote>", options = list(visitor = visitor))

```
