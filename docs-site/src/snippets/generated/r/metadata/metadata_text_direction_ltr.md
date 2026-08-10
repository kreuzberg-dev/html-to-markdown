```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<html lang=\"en\" dir=\"ltr\"><head><title>LTR Document</title></head><body><p>This is left-to-right text.</p></body></html>", options = ConversionOptions$from_json(jsonlite::toJSON(list("extract_metadata" = TRUE), auto_unbox = TRUE)))

```
