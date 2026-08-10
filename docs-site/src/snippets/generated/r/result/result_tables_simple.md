```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<table><thead><tr><th>Name</th><th>Age</th></tr></thead><tbody><tr><td>Alice</td><td>30</td></tr></tbody></table>", options = ConversionOptions$from_json(jsonlite::toJSON(list("include_document_structure" = TRUE), auto_unbox = TRUE)))

```
