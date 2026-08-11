---
id: fixture_r_whitespace_only
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>   </p>", options = ConversionOptions$default())

```
