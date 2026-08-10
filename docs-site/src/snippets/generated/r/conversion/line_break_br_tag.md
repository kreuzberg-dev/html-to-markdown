```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>First line.<br>Second line.</p>", options = ConversionOptions$default())

```
