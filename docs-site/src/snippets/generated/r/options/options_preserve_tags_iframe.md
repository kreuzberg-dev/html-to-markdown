```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Before</p><iframe src='video.html' width='560'></iframe><p>After</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("preserve_tags" = I(c("iframe"))), auto_unbox = TRUE)))

```
