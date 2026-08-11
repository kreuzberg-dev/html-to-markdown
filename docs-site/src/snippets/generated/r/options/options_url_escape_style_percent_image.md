---
id: fixture_r_options_url_escape_style_percent_image
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<img src=\"/img (1) <draft>.png\" alt=\"alt\">", options = ConversionOptions$from_json(jsonlite::toJSON(list("url_escape_style" = "percent"), auto_unbox = TRUE)))

```
