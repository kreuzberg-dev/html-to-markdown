```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>See the chart below:</p><figure><img src=\"chart.svg\"><figcaption>Revenue Trends 2020-2024</figcaption></figure><p>As shown in the chart above.</p>", ConversionOptions())
}

```
