---
id: fixture_r_options_list_indent_tabs
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<ul><li>Parent<ul><li>Child</li></ul></li></ul>", options = ConversionOptions$from_json(jsonlite::toJSON(list("list_indent_type" = "tabs"), auto_unbox = TRUE)))

```
