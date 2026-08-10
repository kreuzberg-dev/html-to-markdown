```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>This is a long paragraph that should not be wrapped at all because wrapping is disabled.</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("wrap" = FALSE), auto_unbox = TRUE)))

```
