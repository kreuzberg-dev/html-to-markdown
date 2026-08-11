---
id: fixture_r_style_tags_only
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<html><head><style>body { color: red; }</style></head><body><style>.foo { margin: 0; }</style></body></html>", options = ConversionOptions$default())

```
