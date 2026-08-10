```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorAudioCustom: HtmlVisitorProtocol {
    func visitAudio(_ ctx: HtmlToMarkdown.NodeContext, _ src: String?) -> VisitResult { return .custom(field0: "[AUDIO: podcast.mp3]") }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorAudioCustom())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<p>Listen to this: <audio src=\"podcast.mp3\" controls></audio></p>", options: _options)

```
