```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<img src=\"chart.png\" alt=\"Sales chart\" title=\"Q3 Sales\">", options = ConversionOptions$default())

```
