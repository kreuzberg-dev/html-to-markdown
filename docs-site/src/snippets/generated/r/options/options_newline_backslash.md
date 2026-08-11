---
id: fixture_r_options_newline_backslash
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Line one<br>Line two</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("newline_style" = "backslash"), auto_unbox = TRUE)))

```
