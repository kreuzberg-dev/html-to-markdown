```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<h1>One</h1>\n<!-- /// --->\n<p>Two</p>", ConversionOptions())
}

```
