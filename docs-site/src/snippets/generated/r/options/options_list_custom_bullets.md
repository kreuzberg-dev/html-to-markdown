---
id: fixture_r_options_list_custom_bullets
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<ul><li>Item A</li><li>Item B</li></ul>", options = ConversionOptions$from_json(jsonlite::toJSON(list("bullets" = "*"), auto_unbox = TRUE)))

```
