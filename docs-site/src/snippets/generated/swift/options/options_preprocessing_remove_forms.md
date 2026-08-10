```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"preprocessing\":{\"remove_forms\":true}}")
_ = try HtmlToMarkdown.convert(html: "<p>Before</p><form><input type='text'/><button>Submit</button></form><p>After</p>", options: _options)

```
