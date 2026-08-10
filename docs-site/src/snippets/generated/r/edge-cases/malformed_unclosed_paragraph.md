```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>This paragraph is never closed", options = ConversionOptions$default())

```
