---
id: fixture_r_malformed_overlapping_tags
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p><b><i>bold and italic</b></i></p>", options = ConversionOptions$default())

```
