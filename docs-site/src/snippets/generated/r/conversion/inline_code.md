---
id: fixture_r_inline_code
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Use <code>console.log()</code> to debug</p>", options = ConversionOptions$default())

```
