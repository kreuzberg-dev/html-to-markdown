```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>x<sup>2</sup></p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("sup_symbol" = "^"), auto_unbox = TRUE)))

```
