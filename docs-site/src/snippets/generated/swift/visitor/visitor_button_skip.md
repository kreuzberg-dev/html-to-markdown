```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorButtonSkip: HtmlVisitorProtocol {
    func visitButton(_ ctx: HtmlToMarkdown.NodeContext, _ text: String) -> VisitResult { return .skip }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorButtonSkip())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<p>Actions available: <button>Save</button> <button>Delete</button> <button>Export</button></p>", options: _options)

```
