```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Before SVG.</p><svg xmlns=\"http://www.w3.org/2000/svg\"><script>alert('svg-xss')</script><text>SVG text</text></svg><p>After SVG.</p>", options = ConversionOptions$default())

```
