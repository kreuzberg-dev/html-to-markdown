```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<div><p>Shallow</p><div><div><div><p>Too deep</p></div></div></div></div>", options = ConversionOptions$from_json(jsonlite::toJSON(list("max_depth" = 3), auto_unbox = TRUE)))

```
