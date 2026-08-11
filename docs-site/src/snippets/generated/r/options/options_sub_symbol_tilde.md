---
id: fixture_r_options_sub_symbol_tilde
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>H<sub>2</sub>O</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("sub_symbol" = "~"), auto_unbox = TRUE)))

```
