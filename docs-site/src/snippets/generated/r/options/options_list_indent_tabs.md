```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<ul><li>Parent<ul><li>Child</li></ul></li></ul>", options = ConversionOptions$from_json(jsonlite::toJSON(list("list_indent_type" = "tabs"), auto_unbox = TRUE)))

```
