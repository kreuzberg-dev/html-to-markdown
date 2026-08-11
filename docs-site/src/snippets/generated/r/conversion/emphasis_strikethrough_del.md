---
id: fixture_r_emphasis_strikethrough_del
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p><del>deleted text</del></p>", options = ConversionOptions$default())

```
