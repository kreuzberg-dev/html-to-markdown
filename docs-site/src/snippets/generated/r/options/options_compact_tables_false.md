---
id: fixture_r_options_compact_tables_false
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<table><thead><tr><th>Name</th><th>Score</th></tr></thead><tbody><tr><td>Alice</td><td>100</td></tr><tr><td>Bob</td><td>42</td></tr></tbody></table>", options = ConversionOptions$from_json(jsonlite::toJSON(list("compact_tables" = FALSE), auto_unbox = TRUE)))

```
