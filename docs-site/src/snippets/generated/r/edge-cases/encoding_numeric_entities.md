```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Copyright: &#169; Trade: &#174; Euro: &#8364; Hex: &#x00A9;</p>", options = ConversionOptions$default())

```
