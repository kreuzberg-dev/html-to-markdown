```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<a href=\"foobar.png\">foobar.png</a>", options = ConversionOptions$default())

```
