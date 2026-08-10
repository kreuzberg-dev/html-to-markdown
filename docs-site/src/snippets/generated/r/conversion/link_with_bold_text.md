```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<a href=\"https://example.com\"><strong>Bold link</strong></a>", options = ConversionOptions$default())

```
