```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Intro text</p><pre><code>let x = 42;</code></pre><p>Outro text</p>", ConversionOptions())
}

```
