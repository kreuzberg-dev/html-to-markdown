```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Use <code>console.log()</code> to debug</p>", options = ConversionOptions$default())

```
