```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Contact <a href=\"mailto:hello@example.com\">us</a> directly.</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("extract_metadata" = TRUE), auto_unbox = TRUE)))

```
