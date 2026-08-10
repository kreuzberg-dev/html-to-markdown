```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_custom_element = function(ctx, tag_name, html) {
    "PreserveHtml"
  }
)

result <- convert(html = "<article><p>Article text</p><x-custom>Custom element with content</x-custom><p>More article text</p></article>", options = list(visitor = visitor))

```
