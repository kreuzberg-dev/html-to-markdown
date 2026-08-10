```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p><u>underlined</u></p>", options = ConversionOptions$default())

```
