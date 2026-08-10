```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"preprocessing\":{\"remove_forms\":false}}")
_ = try HtmlToMarkdown.convert(html: "<form><label for=\"name\">Name:</label><input type=\"text\" id=\"name\" placeholder=\"Enter name\"></form>", options: _options)

```
