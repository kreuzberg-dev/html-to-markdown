```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Tiny limit: <img src=\"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==\" alt=\"pixel\"></p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("extract_images" = TRUE, "max_image_size" = 10), auto_unbox = TRUE)))

```
