---
id: fixture_r_options_url_escape_style_percent_angle_brackets_in_url
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<a href=\"/file (1) <draft>.pdf\">file</a>", options = ConversionOptions$from_json(jsonlite::toJSON(list("url_escape_style" = "percent"), auto_unbox = TRUE)))

```
