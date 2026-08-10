```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>First<br>Second</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("newline_style" = "spaces"), auto_unbox = TRUE)))

```
