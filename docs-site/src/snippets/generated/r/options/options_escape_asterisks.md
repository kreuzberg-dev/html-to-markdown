---
id: fixture_r_options_escape_asterisks
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Use 2*3 = 6 in math.</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("escape_asterisks" = TRUE), auto_unbox = TRUE)))

```
