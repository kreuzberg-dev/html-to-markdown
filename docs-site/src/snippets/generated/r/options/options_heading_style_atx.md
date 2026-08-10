```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<h1>Title</h1><h2>Subtitle</h2>", options = ConversionOptions$from_json(jsonlite::toJSON(list("heading_style" = "atx"), auto_unbox = TRUE)))

```
