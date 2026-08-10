```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<nav>NavSection</nav><p>Paragraph</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("preprocessing" = list("enabled" = FALSE)), auto_unbox = TRUE)))

```
