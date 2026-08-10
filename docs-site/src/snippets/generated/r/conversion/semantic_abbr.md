```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>The <abbr title=\"World Wide Web\">WWW</abbr> is global.</p>", options = ConversionOptions$default())

```
