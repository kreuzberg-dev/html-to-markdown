---
id: fixture_r_paragraph_multiple
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>First paragraph.</p><p>Second paragraph.</p>", options = ConversionOptions$default())

```
