```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<dl><dt>Term One</dt><dd>Definition of term one.</dd><dt>Term Two</dt><dd>Definition of term two.</dd></dl>", options = ConversionOptions$default())

```
