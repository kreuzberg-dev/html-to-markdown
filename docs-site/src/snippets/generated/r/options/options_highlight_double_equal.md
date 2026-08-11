---
id: fixture_r_options_highlight_double_equal
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Text with <mark>highlighted</mark> here.</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("highlight_style" = "double_equal"), auto_unbox = TRUE)))

```
