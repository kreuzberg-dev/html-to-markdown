```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<table><thead><tr><th colspan=\"2\">Full Name</th></tr></thead><tbody><tr><td>John</td><td>Doe</td></tr></tbody></table>", options = ConversionOptions$default())

```
