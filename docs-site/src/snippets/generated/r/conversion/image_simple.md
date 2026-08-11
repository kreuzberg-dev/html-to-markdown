---
id: fixture_r_image_simple
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<img src=\"photo.jpg\" alt=\"A photo\">", options = ConversionOptions$default())

```
