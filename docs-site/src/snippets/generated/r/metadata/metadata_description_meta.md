```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<html><head><title>Page</title><meta name=\"description\" content=\"This is the page description.\"></head><body><p>Content</p></body></html>", options = ConversionOptions$from_json(jsonlite::toJSON(list("extract_metadata" = TRUE), auto_unbox = TRUE)))

```
