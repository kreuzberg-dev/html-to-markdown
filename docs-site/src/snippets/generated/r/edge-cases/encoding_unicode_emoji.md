```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Hello 🌍 World 🚀</p><p>Stars: ⭐ ✨</p>", options = ConversionOptions$default())

```
