```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<html><head><title>Fallback Title</title><meta property=\"og:title\" content=\"OG Title\"><meta property=\"og:description\" content=\"OG description text.\"><meta property=\"og:image\" content=\"https://example.com/image.jpg\"></head><body><p>Content</p></body></html>", options = ConversionOptions$default())

```
