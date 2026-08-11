---
id: fixture_r_emphasis_mark_highlight
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p><mark>highlighted</mark></p>", options = ConversionOptions$default())

```
