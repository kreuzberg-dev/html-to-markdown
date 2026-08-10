```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Jump to <a href=\"#section\">section</a> below.</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("extract_metadata" = TRUE), auto_unbox = TRUE)))

```
