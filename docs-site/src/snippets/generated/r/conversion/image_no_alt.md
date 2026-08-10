```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<img src=\"banner.jpg\">", options = ConversionOptions$default())

```
