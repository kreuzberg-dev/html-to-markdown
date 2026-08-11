---
id: fixture_r_code_with_backticks_in_content
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Use <code>`backtick` here</code> carefully.</p>", options = ConversionOptions$default())

```
