```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<h1>Chapter One</h1><p>Chapter intro.</p><h2>Section One</h2><p>Section content.</p>", options)
}

```
