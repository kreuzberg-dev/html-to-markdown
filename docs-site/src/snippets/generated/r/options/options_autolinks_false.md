```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p><a href='https://example.com'>https://example.com</a></p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("autolinks" = FALSE), auto_unbox = TRUE)))

```
