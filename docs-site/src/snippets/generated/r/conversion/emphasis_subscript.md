```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>H<sub>2</sub>O</p>", options = ConversionOptions$default())

```
