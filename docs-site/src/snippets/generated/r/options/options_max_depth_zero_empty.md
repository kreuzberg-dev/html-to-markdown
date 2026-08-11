---
id: fixture_r_options_max_depth_zero_empty
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Hello</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("max_depth" = 0), auto_unbox = TRUE)))

```
