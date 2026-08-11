---
id: fixture_r_link_image_inside
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<a href=\"https://example.com\"><img src=\"logo.png\" alt=\"Logo\"></a>", options = ConversionOptions$default())

```
