```r title="R"
library("htmltomarkdown", character.only = TRUE)

result <- convert(html = "<pre><code class=\"language-python\">print('hello')</code></pre>", options = ConversionOptions$default())

```
