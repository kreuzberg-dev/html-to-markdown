```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<a href=\"https://example.com\" title=\"Example Site\">Example</a>", options = ConversionOptions$default())

```
