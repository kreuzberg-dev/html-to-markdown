```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<a href=\"/file (1) <draft>.pdf\">file</a>", options = ConversionOptions$from_json(jsonlite::toJSON(list("url_escape_style" = "percent"), auto_unbox = TRUE)))

```
