```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Embedded map:</p><iframe src=\"https://maps.example.com/embed\" width=\"400\" height=\"300\"></iframe><p>End of map</p>", ConversionOptions())
}

```
