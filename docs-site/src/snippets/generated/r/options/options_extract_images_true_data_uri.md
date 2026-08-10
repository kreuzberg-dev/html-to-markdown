```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Before<img src=\"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==\" alt=\"pixel\">After</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("extract_images" = TRUE), auto_unbox = TRUE)))

```
