```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p><img src=\"https://example.com/photo.jpg\" alt=\"A photo\"></p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("extract_metadata" = TRUE), auto_unbox = TRUE)))

```
