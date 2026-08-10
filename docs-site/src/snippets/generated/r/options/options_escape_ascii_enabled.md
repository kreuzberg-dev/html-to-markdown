```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Text with # hash and [brackets] and * star</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("escape_ascii" = TRUE), auto_unbox = TRUE)))

```
