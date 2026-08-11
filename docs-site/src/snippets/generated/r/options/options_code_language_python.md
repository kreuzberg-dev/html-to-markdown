---
id: fixture_r_options_code_language_python
language: r
target: r
level: typecheck
requires: []
side_effect: safe
---

```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<pre><code>def hello(): pass</code></pre>", options = ConversionOptions$from_json(jsonlite::toJSON(list("code_language" = "python"), auto_unbox = TRUE)))

```
