```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<body><nav class=\"nav\">Menu</nav><p>Content</p><footer>Footer</footer></body>", options = ConversionOptions$from_json(jsonlite::toJSON(list("exclude_selectors" = I(c(".nav", "footer"))), auto_unbox = TRUE)))

```
