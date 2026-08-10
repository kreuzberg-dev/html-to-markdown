```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<table><tr><th>X</th></tr><tr><td>Y</td></tr></table>", options = ConversionOptions$default())

```
