```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorUnknownTagPreservation: HtmlVisitorProtocol {
    func visitCustomElement(_ ctx: HtmlToMarkdown.NodeContext, _ tagName: String, _ html: String) -> VisitResult { return .preserveHtml }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorUnknownTagPreservation())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<article><p>Article text</p><x-custom>Custom element with content</x-custom><p>More article text</p></article>", options: _options)

```
