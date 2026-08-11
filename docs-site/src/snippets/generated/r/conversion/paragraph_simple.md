---
id: fixture_r_paragraph_simple
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Hello World</p>", options = ConversionOptions$default())

```
