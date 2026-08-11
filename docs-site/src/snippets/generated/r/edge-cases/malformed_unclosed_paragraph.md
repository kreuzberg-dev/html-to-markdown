---
id: fixture_r_malformed_unclosed_paragraph
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>This paragraph is never closed", options = ConversionOptions$default())

```
