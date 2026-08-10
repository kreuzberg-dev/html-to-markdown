```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Unclosed paragraph<div>Mixed nesting</p></div>", options = ConversionOptions$default())

```
