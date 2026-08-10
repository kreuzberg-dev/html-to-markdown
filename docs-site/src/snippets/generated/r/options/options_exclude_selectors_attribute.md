```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<body><div role=\"complementary\">Sidebar</div><p>Primary text</p></body>", options = ConversionOptions$from_json(jsonlite::toJSON(list("exclude_selectors" = I(c("[role='complementary']"))), auto_unbox = TRUE)))

```
