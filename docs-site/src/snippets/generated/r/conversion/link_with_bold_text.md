---
id: fixture_r_link_with_bold_text
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<a href=\"https://example.com\"><strong>Bold link</strong></a>", options = ConversionOptions$default())

```
