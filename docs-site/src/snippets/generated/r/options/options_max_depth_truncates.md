---
id: fixture_r_options_max_depth_truncates
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<div><p>Shallow</p><div><div><div><p>Too deep</p></div></div></div></div>", options = ConversionOptions$from_json(jsonlite::toJSON(list("max_depth" = 3), auto_unbox = TRUE)))

```
