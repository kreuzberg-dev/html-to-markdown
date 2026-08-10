```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Line one.<br>Line two.<br>Line three.</p>", options = ConversionOptions$default())

```
