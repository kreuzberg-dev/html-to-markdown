```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<p>Foo</p><pre><code>1\n2\n</code></pre><p>Bar</p>", options = ConversionOptions$from_json(jsonlite::toJSON(list("code_block_style" = "backticks"), auto_unbox = TRUE)))

```
