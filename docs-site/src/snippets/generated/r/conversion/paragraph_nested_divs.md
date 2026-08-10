```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<div><div><p>Nested text</p></div></div>", options = ConversionOptions$default())

```
