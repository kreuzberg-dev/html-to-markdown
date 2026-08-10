```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<a href=\"#section\">Jump to section</a>", options = ConversionOptions$default())

```
