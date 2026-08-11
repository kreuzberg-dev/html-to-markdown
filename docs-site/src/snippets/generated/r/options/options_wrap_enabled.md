---
id: fixture_r_options_wrap_enabled
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>This is a long paragraph that should be wrapped at the specified column width when the wrap option is enabled.</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("wrap" = TRUE, "wrap_width" = 40), auto_unbox = TRUE)))

```
