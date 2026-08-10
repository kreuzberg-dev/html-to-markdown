```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<!-- This is a comment --><!-- Another comment -->", options = ConversionOptions$default())

```
