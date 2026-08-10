```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<ul><li>Item A</li><li>Item B</li></ul>", options = ConversionOptions$from_json(jsonlite::toJSON(list("bullets" = "*"), auto_unbox = TRUE)))

```
