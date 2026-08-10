```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"skip_images\":true}")
_ = try HtmlToMarkdown.convert(html: "<p>Before <img src='test.jpg' alt='photo'> After</p>", options: _options)

```
