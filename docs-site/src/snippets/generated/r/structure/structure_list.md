```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Items:</p><ul><li>Alpha</li><li>Beta</li><li>Gamma</li></ul>", options = ConversionOptions$from_json(jsonlite::toJSON(list("include_document_structure" = TRUE), auto_unbox = TRUE)))

```
