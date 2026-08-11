---
id: fixture_r_image_linked
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<a href=\"https://example.com\"><img src=\"icon.png\" alt=\"Icon\"></a>", options = ConversionOptions$default())

```
