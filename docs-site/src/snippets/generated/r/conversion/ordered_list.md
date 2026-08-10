```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<ol><li>First</li><li>Second</li><li>Third</li></ol>", options = ConversionOptions$default())

```
