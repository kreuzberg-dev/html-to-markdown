---
id: fixture_r_heading_h2
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<h2>Heading 2</h2>", options = ConversionOptions$default())

```
