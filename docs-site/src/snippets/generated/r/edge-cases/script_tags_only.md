```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<html><head><script>alert('xss')</script></head><body><script>document.write('hello')</script></body></html>", options = ConversionOptions$default())

```
