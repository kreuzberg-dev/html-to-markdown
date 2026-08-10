```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>This is <mark>highlighted text</mark> in a sentence.</p>", options = ConversionOptions$default())

```
