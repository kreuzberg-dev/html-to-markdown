```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p><a href=\"https://example.com\" onclick=\"alert('xss')\">Click me</a></p><button onmouseover=\"steal_data()\">Hover me</button>", options = ConversionOptions$default())

```
