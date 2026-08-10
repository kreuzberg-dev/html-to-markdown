```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<blockquote><p>First paragraph.</p><p>Second paragraph.</p></blockquote>", options = ConversionOptions$default())

```
