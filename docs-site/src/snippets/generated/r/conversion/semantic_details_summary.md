```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<details><summary>Click to expand</summary><p>Hidden content here.</p></details>", options = ConversionOptions$default())

```
