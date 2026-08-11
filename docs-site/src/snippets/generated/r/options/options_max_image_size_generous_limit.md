---
id: fixture_r_options_max_image_size_generous_limit
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Image: <img src=\"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==\" alt=\"pixel\"></p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("extract_images" = TRUE, "max_image_size" = 10485760), auto_unbox = TRUE)))

```
