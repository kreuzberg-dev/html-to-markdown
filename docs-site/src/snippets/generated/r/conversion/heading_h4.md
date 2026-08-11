---
id: fixture_r_heading_h4
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<h4>Heading 4</h4>", options = ConversionOptions$default())

```
