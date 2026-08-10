```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p><s>strikethrough</s></p>", options = ConversionOptions$default())

```
