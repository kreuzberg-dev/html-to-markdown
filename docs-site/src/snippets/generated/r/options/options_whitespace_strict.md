---
id: fixture_r_options_whitespace_strict
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Preserved   spacing.</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("whitespace_mode" = "strict"), auto_unbox = TRUE)))

```
