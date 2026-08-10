```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p><em>italic</em></p>", options = ConversionOptions$default())

```
