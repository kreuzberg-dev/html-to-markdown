---
id: fixture_r_options_output_format_plain
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<h1>Title</h1><p>Some <strong>bold</strong> text.</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("output_format" = "plain"), auto_unbox = TRUE)))

```
