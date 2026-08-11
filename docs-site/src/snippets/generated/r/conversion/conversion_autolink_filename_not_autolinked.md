---
id: fixture_r_conversion_autolink_filename_not_autolinked
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<a href=\"foobar.png\">foobar.png</a>", options = ConversionOptions$default())

```
