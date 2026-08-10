```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<pre><code>let x = 1;</code></pre>", options = ConversionOptions$from_json(jsonlite::toJSON(list("code_block_style" = "tildes"), auto_unbox = TRUE)))

```
