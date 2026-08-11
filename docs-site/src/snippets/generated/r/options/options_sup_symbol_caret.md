---
id: fixture_r_options_sup_symbol_caret
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>x<sup>2</sup></p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("sup_symbol" = "^"), auto_unbox = TRUE)))

```
