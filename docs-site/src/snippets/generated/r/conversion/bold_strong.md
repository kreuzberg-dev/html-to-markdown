```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p><strong>bold</strong></p>", options = ConversionOptions$default())

```
