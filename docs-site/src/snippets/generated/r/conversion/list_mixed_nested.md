```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<ul><li>Item A<ol><li>Sub 1</li><li>Sub 2</li></ol></li><li>Item B</li></ul>", options = ConversionOptions$default())

```
