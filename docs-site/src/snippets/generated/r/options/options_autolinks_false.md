---
id: fixture_r_options_autolinks_false
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p><a href='https://example.com'>https://example.com</a></p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("autolinks" = FALSE), auto_unbox = TRUE)))

```
