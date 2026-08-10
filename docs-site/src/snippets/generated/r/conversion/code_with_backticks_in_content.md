```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Use <code>`backtick` here</code> carefully.</p>", options = ConversionOptions$default())

```
