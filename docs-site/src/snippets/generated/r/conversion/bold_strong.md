---
id: fixture_r_bold_strong
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p><strong>bold</strong></p>", options = ConversionOptions$default())

```
