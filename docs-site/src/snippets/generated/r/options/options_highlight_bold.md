---
id: fixture_r_options_highlight_bold
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Text with <mark>highlighted</mark> text.</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("highlight_style" = "bold"), auto_unbox = TRUE)))

```
