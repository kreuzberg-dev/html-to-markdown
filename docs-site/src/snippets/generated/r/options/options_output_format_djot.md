---
id: fixture_r_options_output_format_djot
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Simple paragraph.</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("output_format" = "djot"), auto_unbox = TRUE)))

```
