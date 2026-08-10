```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<blockquote><p>Quote text</p></blockquote>", options = ConversionOptions$default())

```
