---
id: fixture_r_options_wrap_disabled
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>This is a long paragraph that should not be wrapped at all because wrapping is disabled.</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("wrap" = FALSE), auto_unbox = TRUE)))

```
