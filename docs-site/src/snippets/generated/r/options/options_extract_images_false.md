```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Text with <img src=\"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==\" alt=\"pixel\"> image.</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("extract_images" = FALSE), auto_unbox = TRUE)))

```
