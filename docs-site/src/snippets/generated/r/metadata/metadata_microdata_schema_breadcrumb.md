```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<html><head><title>Navigation</title></head><body><nav itemscope itemtype=\"https://schema.org/BreadcrumbList\"><span itemprop=\"itemListElement\" itemscope itemtype=\"https://schema.org/ListItem\"><a itemprop=\"item\" href=\"https://example.com\"><span itemprop=\"name\">Home</span></a></span><span itemprop=\"itemListElement\" itemscope itemtype=\"https://schema.org/ListItem\"><a itemprop=\"item\" href=\"https://example.com/products\"><span itemprop=\"name\">Products</span></a></span><span itemprop=\"itemListElement\" itemscope itemtype=\"https://schema.org/ListItem\"><span itemprop=\"name\">Current Page</span></span></nav></body></html>", options = ConversionOptions$from_json(jsonlite::toJSON(list("extract_metadata" = TRUE, "preprocessing" = list("remove_navigation" = FALSE)), auto_unbox = TRUE)))

```
