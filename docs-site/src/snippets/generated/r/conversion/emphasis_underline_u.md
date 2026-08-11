---
id: fixture_r_emphasis_underline_u
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p><u>underlined</u></p>", options = ConversionOptions$default())

```
