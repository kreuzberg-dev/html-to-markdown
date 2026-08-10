```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<a href=\"https://example.com\"><img src=\"logo.png\" alt=\"Logo\"></a>", options = ConversionOptions$default())

```
