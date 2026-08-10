```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<h1>Top Level</h1><p>Top intro.</p><h2>Mid Level</h2><p>Mid content.</p><h3>Deep Level</h3><p>Deep content.</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("include_document_structure" = TRUE), auto_unbox = TRUE)))

```
