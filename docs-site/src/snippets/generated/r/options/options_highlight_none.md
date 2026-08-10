```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Text with <mark>plain</mark> content.</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("highlight_style" = "none"), auto_unbox = TRUE)))

```
