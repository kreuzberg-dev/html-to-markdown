---
id: fixture_r_visitor_custom_link_format
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_link = function(ctx, href, text, title) {
    list(Custom = paste0(text, " (", href, ")"))
  }
)

result <- convert(html = "<p>Visit <a href=\"https://example.com\">Example</a> for more info.</p>", options = list(visitor = visitor))

```
