```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Simple paragraph.</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("output_format" = "djot"), auto_unbox = TRUE)))

```
