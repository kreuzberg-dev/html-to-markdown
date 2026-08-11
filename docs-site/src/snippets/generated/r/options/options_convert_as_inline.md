---
id: fixture_r_options_convert_as_inline
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>One</p><p>Two</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("convert_as_inline" = TRUE), auto_unbox = TRUE)))

```
