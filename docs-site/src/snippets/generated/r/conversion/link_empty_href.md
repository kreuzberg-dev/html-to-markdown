```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<a href=\"\">No destination</a>", options = ConversionOptions$default())

```
