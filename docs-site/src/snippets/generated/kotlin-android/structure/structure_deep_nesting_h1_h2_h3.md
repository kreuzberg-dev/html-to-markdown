```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<h1>Top Level</h1><p>Top intro.</p><h2>Mid Level</h2><p>Mid content.</p><h3>Deep Level</h3><p>Deep content.</p>", options)
}

```
