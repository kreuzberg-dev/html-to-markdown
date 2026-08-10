```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<figure><img src=\"sunset.jpg\" alt=\"A sunset\"><figcaption>Beautiful sunset over the ocean</figcaption></figure>", options = ConversionOptions$default())

```
