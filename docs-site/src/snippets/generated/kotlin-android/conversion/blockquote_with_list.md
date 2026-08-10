```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<blockquote><p>Quote intro:</p><ul><li>Point one</li><li>Point two</li></ul></blockquote>", ConversionOptions())
}

```
