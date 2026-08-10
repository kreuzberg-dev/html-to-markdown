```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<ol><li>Step 1<ol><li>Step 1a</li><li>Step 1b</li></ol></li><li>Step 2</li></ol>", options = ConversionOptions$default())

```
