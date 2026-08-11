---
id: fixture_r_options_link_style_reference
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p><a href='https://example.com'>Example</a> and <a href='https://other.com'>Other</a></p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("link_style" = "reference"), auto_unbox = TRUE)))

```
