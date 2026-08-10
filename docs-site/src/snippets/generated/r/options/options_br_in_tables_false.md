```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<table><tr><th>Col</th></tr><tr><td>A<br>B</td></tr></table>", options = ConversionOptions$from_json(jsonlite::toJSON(list("br_in_tables" = FALSE), auto_unbox = TRUE)))

```
