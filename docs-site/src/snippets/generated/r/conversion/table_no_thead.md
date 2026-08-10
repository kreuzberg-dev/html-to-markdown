```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<table><tr><td>Product</td><td>Price</td></tr><tr><td>Apple</td><td>1.00</td></tr></table>", options = ConversionOptions$default())

```
