```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<a href=\"/docs/intro.html\">/docs/intro.html</a>", options = ConversionOptions$default())

```
