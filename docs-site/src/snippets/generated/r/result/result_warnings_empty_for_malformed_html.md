---
id: fixture_r_result_warnings_empty_for_malformed_html
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Unclosed paragraph<div>Mixed nesting</p></div>", options = ConversionOptions$default())

```
