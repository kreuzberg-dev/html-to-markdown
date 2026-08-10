```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Above</p><hr><p>Below</p>", options = ConversionOptions$default())

```
