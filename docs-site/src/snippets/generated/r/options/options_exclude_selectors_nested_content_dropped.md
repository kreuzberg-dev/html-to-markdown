```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<body><aside class=\"sidebar\"><h2>Related</h2><p>Sidebar text</p></aside><main><p>Main text</p></main></body>", options = ConversionOptions$from_json(jsonlite::toJSON(list("exclude_selectors" = I(c(".sidebar"))), auto_unbox = TRUE)))

```
