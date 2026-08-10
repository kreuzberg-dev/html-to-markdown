```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<body><div class=\"cookie-banner\">Accept cookies</div><p>Main content</p></body>", options)
}

```
