```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<h1>One</h1>\n<!-- /// --->\n<p>Two</p>", options = ConversionOptions$default())

```
