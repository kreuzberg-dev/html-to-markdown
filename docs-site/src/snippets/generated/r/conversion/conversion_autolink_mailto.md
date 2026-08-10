```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<a href=\"mailto:a@b.com\">a@b.com</a>", options = ConversionOptions$default())

```
