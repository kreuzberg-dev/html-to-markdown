```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Text with <mark>highlighted</mark> text.</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("highlight_style" = "bold"), auto_unbox = TRUE)))

```
