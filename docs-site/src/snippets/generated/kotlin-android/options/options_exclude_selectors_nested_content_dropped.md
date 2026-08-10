```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<body><aside class=\"sidebar\"><h2>Related</h2><p>Sidebar text</p></aside><main><p>Main text</p></main></body>", options)
}

```
