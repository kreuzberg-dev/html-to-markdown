```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>The variable_name is defined.</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("escape_underscores" = TRUE), auto_unbox = TRUE)))

```
