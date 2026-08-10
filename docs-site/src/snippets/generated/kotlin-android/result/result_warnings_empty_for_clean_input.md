```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<h1>Title</h1><p>Clean content with <a href='https://example.com'>a link</a>.</p>", ConversionOptions())
}

```
