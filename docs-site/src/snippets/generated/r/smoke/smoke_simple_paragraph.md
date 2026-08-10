```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Hello World</p>", options = ConversionOptions$default())

```
