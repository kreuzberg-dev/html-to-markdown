---
id: fixture_r_options_strong_em_underscore
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p><strong>bold</strong> and <em>italic</em></p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("strong_em_symbol" = "_"), auto_unbox = TRUE)))

```
