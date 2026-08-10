```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Preserved   spacing.</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("whitespace_mode" = "strict"), auto_unbox = TRUE)))

```
