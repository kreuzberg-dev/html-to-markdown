```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>This is a long paragraph that should be wrapped at the specified column width when the wrap option is enabled.</p>", options)
}

```
