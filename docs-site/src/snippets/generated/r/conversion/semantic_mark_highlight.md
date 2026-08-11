---
id: fixture_r_semantic_mark_highlight
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>This is <mark>highlighted text</mark> in a sentence.</p>", options = ConversionOptions$default())

```
