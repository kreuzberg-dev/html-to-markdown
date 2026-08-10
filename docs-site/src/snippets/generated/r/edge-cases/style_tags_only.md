```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<html><head><style>body { color: red; }</style></head><body><style>.foo { margin: 0; }</style></body></html>", options = ConversionOptions$default())

```
