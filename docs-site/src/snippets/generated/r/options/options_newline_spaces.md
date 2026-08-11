---
id: fixture_r_options_newline_spaces
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>First<br>Second</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("newline_style" = "spaces"), auto_unbox = TRUE)))

```
