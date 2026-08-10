```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<table><tr><th>X</th></tr><tr><td>Y</td></tr></table>", ConversionOptions())
}

```
