---
id: fixture_r_heading_h5
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<h5>Heading 5</h5>", options = ConversionOptions$default())

```
