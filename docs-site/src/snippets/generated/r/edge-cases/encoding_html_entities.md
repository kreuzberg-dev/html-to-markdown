```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>&amp; &lt; &gt; &nbsp; &quot; &apos;</p>", options = ConversionOptions$default())

```
