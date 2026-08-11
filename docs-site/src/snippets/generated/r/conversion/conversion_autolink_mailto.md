---
id: fixture_r_conversion_autolink_mailto
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<a href=\"mailto:a@b.com\">a@b.com</a>", options = ConversionOptions$default())

```
