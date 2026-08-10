```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<table><tr><th>Header</th></tr><tr><td>Line 1<br>Line 2</td></tr></table>", options = ConversionOptions$from_json(jsonlite::toJSON(list("br_in_tables" = TRUE), auto_unbox = TRUE)))

```
