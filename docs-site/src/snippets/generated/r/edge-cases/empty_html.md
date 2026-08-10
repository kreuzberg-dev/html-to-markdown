```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<html><head></head><body></body></html>", options = ConversionOptions$default())

```
