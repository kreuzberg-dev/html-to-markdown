```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<a href=\"foobar.png\">foobar.png</a> <a href=\"https://www.heise.de\">https://www.heise.de</a>", options: _options)

```
