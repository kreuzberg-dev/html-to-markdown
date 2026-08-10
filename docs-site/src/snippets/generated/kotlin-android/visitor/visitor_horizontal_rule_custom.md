```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<h1>Section A</h1><p>Content A</p><hr><h1>Section B</h1><p>Content B</p>", ConversionOptions())
}

```
