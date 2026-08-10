```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<pre><code>print('hello')</code></pre>", options = ConversionOptions$from_json(jsonlite::toJSON(list("code_block_style" = "indented"), auto_unbox = TRUE)))

```
