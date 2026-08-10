```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<a href=\"mailto:user@example.com\">Email us</a>", options = ConversionOptions$default())

```
