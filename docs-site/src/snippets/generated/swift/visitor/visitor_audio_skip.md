```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorAudioSkip: HtmlVisitorProtocol {
    func visitAudio(_ ctx: HtmlToMarkdown.NodeContext, _ src: String?) -> VisitResult { return .skip }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorAudioSkip())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<p>Background music:</p><audio src=\"music.ogg\" autoplay></audio><p>Enjoy!</p>", options: _options)

```
