```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<div><p>Shallow</p><div><div><div><p>Too deep</p></div></div></div></div>", options)
}

```
