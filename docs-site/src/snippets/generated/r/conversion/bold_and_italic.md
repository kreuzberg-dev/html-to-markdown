```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p><strong><em>both</em></strong></p>", options = ConversionOptions$default())

```
