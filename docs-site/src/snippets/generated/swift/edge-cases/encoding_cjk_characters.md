```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<p>中文内容</p><p>日本語テキスト</p><p>한국어 텍스트</p>", options: _options)

```
