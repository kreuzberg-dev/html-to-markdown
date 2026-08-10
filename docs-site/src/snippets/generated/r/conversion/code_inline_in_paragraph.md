```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Call the <code>initialize()</code> method first.</p>", options = ConversionOptions$default())

```
