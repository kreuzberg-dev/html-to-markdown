```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Before rule.</p><hr><p>After rule.</p>", options = ConversionOptions$default())

```
