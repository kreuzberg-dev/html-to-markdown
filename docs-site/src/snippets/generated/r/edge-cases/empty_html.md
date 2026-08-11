---
id: fixture_r_empty_html
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<html><head></head><body></body></html>", options = ConversionOptions$default())

```
