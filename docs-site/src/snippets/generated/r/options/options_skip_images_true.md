---
id: fixture_r_options_skip_images_true
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Before <img src='test.jpg' alt='photo'> After</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("skip_images" = TRUE), auto_unbox = TRUE)))

```
