```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<ul><li>Item 1</li><li>Item 2</li><li>Item 3</li></ul>", options = ConversionOptions$default())

```
