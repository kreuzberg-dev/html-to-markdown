```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>First paragraph.</p><p>Second paragraph.</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("strip_newlines" = TRUE), auto_unbox = TRUE)))

```
