```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<h1>Chapter One</h1><h2>Section A</h2><p>Section A content.</p><h1>Chapter Two</h1><h2>Section B</h2><p>Section B content.</p>", options)
}

```
