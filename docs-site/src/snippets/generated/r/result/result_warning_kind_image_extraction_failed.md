---
id: fixture_r_result_warning_kind_image_extraction_failed
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Text<img src=\"data:BADMIME\" alt=\"broken\">end</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("extract_images" = TRUE), auto_unbox = TRUE)))

```
