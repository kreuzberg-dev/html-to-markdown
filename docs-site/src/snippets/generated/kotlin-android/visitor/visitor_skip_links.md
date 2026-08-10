```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Before <a href=\"https://example.com\">link text</a> after</p>", ConversionOptions())
}

```
