```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Use # and | and ~ in text.</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("escape_misc" = TRUE), auto_unbox = TRUE)))

```
