```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<img src=\"chart.png\" alt=\"Sales chart\" title=\"Q3 Sales\">", ConversionOptions())
}

```
