---
id: fixture_r_link_mailto
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<a href=\"mailto:user@example.com\">Email us</a>", options = ConversionOptions$default())

```
