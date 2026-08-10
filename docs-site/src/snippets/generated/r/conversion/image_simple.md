```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<img src=\"photo.jpg\" alt=\"A photo\">", options = ConversionOptions$default())

```
