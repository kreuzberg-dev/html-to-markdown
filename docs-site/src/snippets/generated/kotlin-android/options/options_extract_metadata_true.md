```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<html><head><title>Test Page</title><meta name='description' content='A test page'></head><body><p>Content</p></body></html>", options)
}

```
