```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Safe content.</p><script>alert('xss')</script><p>More safe content.</p>", options = ConversionOptions$default())

```
