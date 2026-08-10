```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>No tables here</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("include_document_structure" = TRUE), auto_unbox = TRUE)))

```
