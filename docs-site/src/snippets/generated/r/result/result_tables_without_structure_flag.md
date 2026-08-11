---
id: fixture_r_result_tables_without_structure_flag
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<table><tr><th>X</th></tr><tr><td>Y</td></tr></table>", options = ConversionOptions$default())

```
