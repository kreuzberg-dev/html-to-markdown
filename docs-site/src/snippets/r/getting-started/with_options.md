```r
library(htmltomarkdown)

opts <- conversion_options(
  heading_style = "Atx",
  wrap = TRUE,
  wrap_width = 80L
)

result <- convert("<h1>Hello</h1><p>World</p>", opts)
cat(result$content)
```
