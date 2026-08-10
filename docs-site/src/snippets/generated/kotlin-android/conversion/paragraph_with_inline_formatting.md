```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>This has <strong>bold</strong>, <em>italic</em>, and a <a href=\"https://example.com\">link</a>.</p>", ConversionOptions())
}

```
