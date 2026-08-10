```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<a href=\"foobar.png\">foobar.png</a> <a href=\"https://www.heise.de\">https://www.heise.de</a>", options = ConversionOptions$default())

```
