---
id: fixture_r_italic_em
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p><em>italic</em></p>", options = ConversionOptions$default())

```
