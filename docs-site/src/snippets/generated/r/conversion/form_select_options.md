```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<form><label>Color:</label><select><option value=\"red\">Red</option><option value=\"blue\" selected>Blue</option><option value=\"green\">Green</option></select></form>", options = ConversionOptions$from_json(jsonlite::toJSON(list("preprocessing" = list("remove_forms" = FALSE)), auto_unbox = TRUE)))

```
