---
id: fixture_r_options_debug_true
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Debug test</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("debug" = TRUE), auto_unbox = TRUE)))

```
