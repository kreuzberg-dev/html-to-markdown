```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<blockquote><p>Outer quote.</p><blockquote><p>Inner quote.</p></blockquote></blockquote>", ConversionOptions())
}

```
