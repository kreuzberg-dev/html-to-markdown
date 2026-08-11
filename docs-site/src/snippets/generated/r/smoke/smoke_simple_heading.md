---
id: fixture_r_smoke_simple_heading
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<h1>Title</h1>", options = ConversionOptions$default())

```
