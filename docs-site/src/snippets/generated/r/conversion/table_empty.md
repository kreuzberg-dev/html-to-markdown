---
id: fixture_r_table_empty
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<table></table>", options = ConversionOptions$default())

```
