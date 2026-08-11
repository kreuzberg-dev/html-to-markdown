---
id: fixture_r_ordered_list
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<ol><li>First</li><li>Second</li><li>Third</li></ol>", options = ConversionOptions$default())

```
