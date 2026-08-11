---
id: fixture_r_line_break_br_tag
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>First line.<br>Second line.</p>", options = ConversionOptions$default())

```
