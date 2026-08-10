```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<html lang=\"es\"><head><title>Spanish Page</title></head><body><h1>Hola Mundo</h1><p>Este es un documento en español.</p></body></html>", options = ConversionOptions$from_json(jsonlite::toJSON(list("extract_metadata" = TRUE), auto_unbox = TRUE)))

```
