---
id: fixture_r_visitor_skip_links
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
    "Skip"
  }
)

result <- convert(html = "<p>Before <a href=\"https://example.com\">link text</a> after</p>", options = list(visitor = visitor))

```
