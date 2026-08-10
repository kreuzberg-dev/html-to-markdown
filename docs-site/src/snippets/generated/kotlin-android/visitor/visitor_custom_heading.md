```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<h2>Section Title</h2><p>Content below heading.</p>", ConversionOptions())
}

```
