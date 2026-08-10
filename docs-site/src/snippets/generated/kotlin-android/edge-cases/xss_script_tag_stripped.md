```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Safe content.</p><script>alert('xss')</script><p>More safe content.</p>", ConversionOptions())
}

```
