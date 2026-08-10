```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>x<sup>2</sup></p>", options = ConversionOptions$default())

```
