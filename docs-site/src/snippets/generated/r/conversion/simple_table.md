```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<table><thead><tr><th>Name</th><th>Age</th></tr></thead><tbody><tr><td>Alice</td><td>30</td></tr></tbody></table>", options = ConversionOptions$default())

```
