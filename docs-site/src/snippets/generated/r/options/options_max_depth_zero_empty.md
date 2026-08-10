```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Hello</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("max_depth" = 0), auto_unbox = TRUE)))

```
