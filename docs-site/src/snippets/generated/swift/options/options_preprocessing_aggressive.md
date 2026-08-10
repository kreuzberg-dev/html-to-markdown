```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"preprocessing\":{\"preset\":\"Aggressive\"}}")
_ = try HtmlToMarkdown.convert(html: "<nav>Menu</nav><article><h1>Title</h1><p>Content</p></article><aside>Sidebar</aside><footer>Footer</footer>", options: _options)

```
