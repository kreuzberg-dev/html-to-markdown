```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<section><h3>Section Heading</h3><p>Section content.</p></section>", options = ConversionOptions$default())

```
