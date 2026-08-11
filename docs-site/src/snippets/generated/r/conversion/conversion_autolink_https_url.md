---
id: fixture_r_conversion_autolink_https_url
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<a href=\"https://example.com\">https://example.com</a>", options = ConversionOptions$default())

```
