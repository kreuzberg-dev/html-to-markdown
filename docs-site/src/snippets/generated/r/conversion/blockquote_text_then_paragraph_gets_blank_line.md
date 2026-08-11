---
id: fixture_r_blockquote_text_then_paragraph_gets_blank_line
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<blockquote>Just text, then <p>a paragraph</p></blockquote>", options = ConversionOptions$default())

```
