```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"preprocessing\":{\"remove_forms\":false}}")
_ = try HtmlToMarkdown.convert(html: "<form><label>Color:</label><select><option value=\"red\">Red</option><option value=\"blue\" selected>Blue</option><option value=\"green\">Green</option></select></form>", options: _options)

```
