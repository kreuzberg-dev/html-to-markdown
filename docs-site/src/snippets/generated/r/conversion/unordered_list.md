---
id: fixture_r_unordered_list
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<ul><li>Item 1</li><li>Item 2</li><li>Item 3</li></ul>", options = ConversionOptions$default())

```
