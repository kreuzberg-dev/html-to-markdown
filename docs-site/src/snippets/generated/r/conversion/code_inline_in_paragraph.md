---
id: fixture_r_code_inline_in_paragraph
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Call the <code>initialize()</code> method first.</p>", options = ConversionOptions$default())

```
