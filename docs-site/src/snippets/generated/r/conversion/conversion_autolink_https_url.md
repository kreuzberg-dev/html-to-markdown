```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<a href=\"https://example.com\">https://example.com</a>", options = ConversionOptions$default())

```
