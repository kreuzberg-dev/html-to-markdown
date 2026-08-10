```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p><b><i>bold and italic</b></i></p>", options = ConversionOptions$default())

```
