---
id: fixture_r_malformed_bogus_comment_triple_dash
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<h1>One</h1>\n<!-- /// --->\n<p>Two</p>", options = ConversionOptions$default())

```
