---
id: fixture_r_options_preprocessing_remove_forms
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Before</p><form><input type='text'/><button>Submit</button></form><p>After</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("preprocessing" = list("remove_forms" = TRUE)), auto_unbox = TRUE)))

```
