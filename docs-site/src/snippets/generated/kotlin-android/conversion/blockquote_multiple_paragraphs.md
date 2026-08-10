```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<blockquote><p>First paragraph.</p><p>Second paragraph.</p></blockquote>", ConversionOptions())
}

```
