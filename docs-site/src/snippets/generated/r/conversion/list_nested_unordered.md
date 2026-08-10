```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<ul><li>Parent A<ul><li>Child A1</li><li>Child A2</li></ul></li><li>Parent B</li></ul>", options = ConversionOptions$default())

```
