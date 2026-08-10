```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Before <img src='test.jpg' alt='photo'> After</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("skip_images" = TRUE), auto_unbox = TRUE)))

```
