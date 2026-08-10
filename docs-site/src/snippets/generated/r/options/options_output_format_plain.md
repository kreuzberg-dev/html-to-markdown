```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<h1>Title</h1><p>Some <strong>bold</strong> text.</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("output_format" = "plain"), auto_unbox = TRUE)))

```
