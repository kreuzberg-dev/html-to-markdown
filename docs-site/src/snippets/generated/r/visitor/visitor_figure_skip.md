```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_figure_start = function(ctx) {
    "Skip"
  }
)

result <- convert(html = "<p>See the chart below:</p><figure><img src=\"chart.svg\"><figcaption>Revenue Trends 2020-2024</figcaption></figure><p>As shown in the chart above.</p>", options = list(visitor = visitor))

```
