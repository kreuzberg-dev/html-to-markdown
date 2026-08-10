```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<body><div class=\"nav\">Navigation</div><p>Article body</p></body>", options = ConversionOptions$from_json(jsonlite::toJSON(list("exclude_selectors" = I(c(".nav")), "output_format" = "plain"), auto_unbox = TRUE)))

```
