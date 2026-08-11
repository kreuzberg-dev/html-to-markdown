---
id: fixture_r_html_comments_only
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<!-- This is a comment --><!-- Another comment -->", options = ConversionOptions$default())

```
