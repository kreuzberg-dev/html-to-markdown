```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<table><thead><tr><th colspan=\"2\">Full Name</th></tr></thead><tbody><tr><td>John</td><td>Doe</td></tr></tbody></table>", ConversionOptions())
}

```
