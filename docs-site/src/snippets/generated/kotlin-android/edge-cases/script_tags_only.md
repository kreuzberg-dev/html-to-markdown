```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<html><head><script>alert('xss')</script></head><body><script>document.write('hello')</script></body></html>", ConversionOptions())
}

```
