---
id: fixture_r_emphasis_superscript
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>x<sup>2</sup></p>", options = ConversionOptions$default())

```
