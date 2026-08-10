```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Hello world</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("exclude_selectors" = I(list())), auto_unbox = TRUE)))

```
