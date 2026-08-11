---
id: fixture_r_semantic_hr
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Above</p><hr><p>Below</p>", options = ConversionOptions$default())

```
