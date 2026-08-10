```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Items:</p><ul><li>Alpha</li><li>Beta</li><li>Gamma</li></ul>", options)
}

```
