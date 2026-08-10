```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<article><h2>Article Title</h2><p>Article body.</p></article>", options = ConversionOptions$default())

```
