```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<ul><li>Outer<ul><li>Inner</li></ul></li></ul>", options = ConversionOptions$from_json(jsonlite::toJSON(list("list_indent_width" = 4), auto_unbox = TRUE)))

```
