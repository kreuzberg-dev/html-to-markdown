```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>This has <strong>bold</strong>, <em>italic</em>, and a <a href=\"https://example.com\">link</a>.</p>", options = ConversionOptions$default())

```
