---
id: fixture_r_paragraph_with_line_breaks
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Line one.<br>Line two.<br>Line three.</p>", options = ConversionOptions$default())

```
