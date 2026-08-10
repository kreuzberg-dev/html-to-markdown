```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Start.<br><br>End.</p>", options = ConversionOptions$default())

```
