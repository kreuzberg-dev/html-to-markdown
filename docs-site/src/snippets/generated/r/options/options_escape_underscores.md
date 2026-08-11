---
id: fixture_r_options_escape_underscores
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>The variable_name is defined.</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("escape_underscores" = TRUE), auto_unbox = TRUE)))

```
