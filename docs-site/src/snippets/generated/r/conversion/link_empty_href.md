---
id: fixture_r_link_empty_href
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<a href=\"\">No destination</a>", options = ConversionOptions$default())

```
