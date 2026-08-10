```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_video = function(ctx, src) {
    list(Custom = paste0("[VIDEO: ", src, "]"))
  }
)

result <- convert(html = "<p>Watch our tutorial:</p><video src=\"tutorial.mp4\" width=\"320\" height=\"240\" controls></video><p>Great content!</p>", options = list(visitor = visitor))

```
