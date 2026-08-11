---
id: fixture_r_options_url_escape_style_angle_default
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<a href=\"/file (1).pdf\">file</a>", options = ConversionOptions$from_json(jsonlite::toJSON(list("url_escape_style" = "angle"), auto_unbox = TRUE)))

```
