```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<h1>Main Title</h1>", options = ConversionOptions$from_json(jsonlite::toJSON(list("heading_style" = "underlined"), auto_unbox = TRUE)))

```
