---
id: fixture_r_image_with_title
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<img src=\"chart.png\" alt=\"Sales chart\" title=\"Q3 Sales\">", options = ConversionOptions$default())

```
