```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<div><h1>Title<p>First paragraph<p>Second paragraph</div>", ConversionOptions())
}

```
