---
id: fixture_r_blockquote_nested_list_indentation_preserved
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<blockquote><ul><li>item a<ul><li>sub a1</li></ul></li></ul></blockquote>", options = ConversionOptions$default())

```
