```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<blockquote><p>Outer quote.</p><blockquote><p>Inner quote.</p></blockquote></blockquote>", options = ConversionOptions$default())

```
