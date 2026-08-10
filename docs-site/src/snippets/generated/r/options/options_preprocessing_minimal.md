```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<nav>Navigation</nav><p>Content</p><footer>Footer</footer>", options = ConversionOptions$from_json(jsonlite::toJSON(list("preprocessing" = list("preset" = "minimal")), auto_unbox = TRUE)))

```
