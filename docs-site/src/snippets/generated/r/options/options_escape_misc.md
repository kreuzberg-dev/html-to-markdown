---
id: fixture_r_options_escape_misc
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Use # and | and ~ in text.</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("escape_misc" = TRUE), auto_unbox = TRUE)))

```
