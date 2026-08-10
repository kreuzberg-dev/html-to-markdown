```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<ul><li><p>First paragraph in item.</p><p>Second paragraph in item.</p></li><li>Simple item</li></ul>", options = ConversionOptions$default())

```
