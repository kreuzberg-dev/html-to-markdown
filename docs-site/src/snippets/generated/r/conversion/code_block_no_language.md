```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<pre><code>plain code here</code></pre>", options = ConversionOptions$default())

```
