```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<h2>Demo</h2><video src=\"demo.webm\"></video><p>See the demo above.</p>", ConversionOptions())
}

```
