```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<div><div><div><div><p>Deep content</p></div></div></div></div>", options = ConversionOptions$default())

```
