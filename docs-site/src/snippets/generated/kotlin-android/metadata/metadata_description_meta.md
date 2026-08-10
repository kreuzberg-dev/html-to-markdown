```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<html><head><title>Page</title><meta name=\"description\" content=\"This is the page description.\"></head><body><p>Content</p></body></html>", options)
}

```
