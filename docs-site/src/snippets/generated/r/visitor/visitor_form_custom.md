```r title="R"
library("htmltomarkdown", character.only = TRUE)

visitor <- list(
  visit_form = function(ctx, action_url, method) {
    list(Custom = "[FORM PLACEHOLDER]")
  }
)

result <- convert(html = "<div><form action=\"/submit\" method=\"POST\"><label>Name: <input type=\"text\" name=\"name\"></label><button type=\"submit\">Submit</button></form></div>", options = list(visitor = visitor))

```
