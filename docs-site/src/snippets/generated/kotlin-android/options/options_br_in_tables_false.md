```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<table><tr><th>Col</th></tr><tr><td>A<br>B</td></tr></table>", options)
}

```
