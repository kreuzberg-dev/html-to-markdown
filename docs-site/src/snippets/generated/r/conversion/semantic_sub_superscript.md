---
id: fixture_r_semantic_sub_superscript
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>H<sub>2</sub>O and E=mc<sup>2</sup></p>", options = ConversionOptions$default())

```
