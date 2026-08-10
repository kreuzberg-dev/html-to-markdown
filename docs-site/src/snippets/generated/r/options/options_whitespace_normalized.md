```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Text   with    extra   spaces.</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("whitespace_mode" = "normalized"), auto_unbox = TRUE)))

```
