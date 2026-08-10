```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p><mark>highlighted</mark></p>", options = ConversionOptions$default())

```
