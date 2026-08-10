```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>See <a href=\"https://example.com\">Example</a> for details.</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("extract_metadata" = TRUE), auto_unbox = TRUE)))

```
