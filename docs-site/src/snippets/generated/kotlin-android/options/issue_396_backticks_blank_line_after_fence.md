```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Foo</p><pre><code>1\n2\n</code></pre><p>Bar</p>", options)
}

```
