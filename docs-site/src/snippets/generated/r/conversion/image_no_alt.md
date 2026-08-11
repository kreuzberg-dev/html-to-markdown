---
id: fixture_r_image_no_alt
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<img src=\"banner.jpg\">", options = ConversionOptions$default())

```
