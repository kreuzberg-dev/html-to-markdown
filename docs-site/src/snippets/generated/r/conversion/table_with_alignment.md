```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<table><thead><tr><th align=\"left\">Left</th><th align=\"center\">Center</th><th align=\"right\">Right</th></tr></thead><tbody><tr><td>L</td><td>C</td><td>R</td></tr></tbody></table>", options = ConversionOptions$default())

```
