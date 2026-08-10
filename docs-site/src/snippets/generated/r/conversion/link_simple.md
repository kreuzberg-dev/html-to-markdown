```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<a href=\"https://example.com\">Example</a>", options = ConversionOptions$default())

```
