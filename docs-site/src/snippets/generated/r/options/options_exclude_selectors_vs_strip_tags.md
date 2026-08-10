```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<body><div class=\"wrapper\"><p>Inner paragraph</p></div><p>Outer text</p></body>", options = ConversionOptions$from_json(jsonlite::toJSON(list("exclude_selectors" = I(c(".wrapper"))), auto_unbox = TRUE)))

```
