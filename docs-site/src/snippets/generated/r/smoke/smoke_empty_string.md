---
id: fixture_r_smoke_empty_string
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "", options = ConversionOptions$default())

```
