```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<h1>Closed Heading</h1>", options = ConversionOptions$from_json(jsonlite::toJSON(list("heading_style" = "atx_closed"), auto_unbox = TRUE)))

```
