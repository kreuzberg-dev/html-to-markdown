---
id: fixture_r_bold_and_italic
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p><strong><em>both</em></strong></p>", options = ConversionOptions$default())

```
