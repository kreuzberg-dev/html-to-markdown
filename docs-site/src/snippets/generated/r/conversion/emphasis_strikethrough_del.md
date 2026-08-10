```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p><del>deleted text</del></p>", options = ConversionOptions$default())

```
