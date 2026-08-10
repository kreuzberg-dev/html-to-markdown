```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<table><tr><th>Header</th></tr><tr><td>Line 1<br>Line 2</td></tr></table>", options)
}

```
