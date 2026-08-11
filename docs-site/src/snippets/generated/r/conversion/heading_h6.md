---
id: fixture_r_heading_h6
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<h6>Heading 6</h6>", options = ConversionOptions$default())

```
