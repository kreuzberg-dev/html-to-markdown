```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<html><head><title>Article</title><script type=\"application/ld+json\">{\"@context\":\"https://schema.org\",\"@type\":\"Article\",\"headline\":\"My Article\",\"author\":{\"@type\":\"Person\",\"name\":\"Jane Doe\"},\"datePublished\":\"2024-01-15\"}</script></head><body><h1>My Article</h1><p>Article body text.</p></body></html>", options = ConversionOptions$default())

```
