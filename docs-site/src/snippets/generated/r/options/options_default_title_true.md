---
id: fixture_r_options_default_title_true
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p><a href='https://example.com'>Link</a></p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("default_title" = TRUE), auto_unbox = TRUE)))

```
