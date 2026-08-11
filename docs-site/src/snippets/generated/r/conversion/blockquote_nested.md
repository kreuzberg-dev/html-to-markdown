---
id: fixture_r_blockquote_nested
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<blockquote><p>Outer quote.</p><blockquote><p>Inner quote.</p></blockquote></blockquote>", options = ConversionOptions$default())

```
