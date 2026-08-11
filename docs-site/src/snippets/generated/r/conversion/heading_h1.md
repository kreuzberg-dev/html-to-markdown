---
id: fixture_r_heading_h1
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<h1>Heading 1</h1>", options = ConversionOptions$default())

```
