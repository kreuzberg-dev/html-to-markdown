---
id: fixture_r_blockquote_multiple_paragraphs
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<blockquote><p>First paragraph.</p><p>Second paragraph.</p></blockquote>", options = ConversionOptions$default())

```
