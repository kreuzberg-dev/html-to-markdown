```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<table><thead><tr><th>Expression</th><th>Result</th></tr></thead><tbody><tr><td>a | b</td><td>true</td></tr></tbody></table>", ConversionOptions())
}

```
