```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<img src=\"/img (1) <draft>.png\" alt=\"alt\">", options = ConversionOptions$from_json(jsonlite::toJSON(list("url_escape_style" = "percent"), auto_unbox = TRUE)))

```
