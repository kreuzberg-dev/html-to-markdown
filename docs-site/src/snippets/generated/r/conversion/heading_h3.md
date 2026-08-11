---
id: fixture_r_heading_h3
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<h3>Heading 3</h3>", options = ConversionOptions$default())

```
