```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<article><h1>Heading</h1><p>Paragraph body.</p></article>", options = ConversionOptions$from_json(jsonlite::toJSON(list("include_document_structure" = FALSE), auto_unbox = TRUE)))

```
