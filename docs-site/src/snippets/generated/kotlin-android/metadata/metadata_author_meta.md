```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<html><head><title>Page</title><meta name=\"author\" content=\"Jane Doe\"></head><body><p>Content</p></body></html>", options)
}

```
