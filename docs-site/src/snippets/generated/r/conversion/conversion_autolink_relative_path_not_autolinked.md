---
id: fixture_r_conversion_autolink_relative_path_not_autolinked
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<a href=\"/docs/intro.html\">/docs/intro.html</a>", options = ConversionOptions$default())

```
