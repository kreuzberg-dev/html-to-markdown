---
id: fixture_r_form_input_elements
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<form><label for=\"name\">Name:</label><input type=\"text\" id=\"name\" placeholder=\"Enter name\"></form>", options = ConversionOptions$from_json(jsonlite::toJSON(list("preprocessing" = list("remove_forms" = FALSE)), auto_unbox = TRUE)))

```
