```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<div><h1>Title<p>First paragraph<p>Second paragraph</div>", options = ConversionOptions$default())

```
