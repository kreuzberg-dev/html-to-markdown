```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<form><label>Message:</label><textarea>Default text content</textarea></form>", options = ConversionOptions$from_json(jsonlite::toJSON(list("preprocessing" = list("remove_forms" = FALSE)), auto_unbox = TRUE)))

```
