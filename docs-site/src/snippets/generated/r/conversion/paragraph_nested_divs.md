---
id: fixture_r_paragraph_nested_divs
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<div><div><p>Nested text</p></div></div>", options = ConversionOptions$default())

```
