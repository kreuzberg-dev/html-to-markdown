---
id: fixture_r_options_encoding_utf8
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Café naïve résumé</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("encoding" = "utf-8"), auto_unbox = TRUE)))

```
