```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<a href=\"https://example.com\"><img src=\"icon.png\" alt=\"Icon\"></a>", options = ConversionOptions$default())

```
