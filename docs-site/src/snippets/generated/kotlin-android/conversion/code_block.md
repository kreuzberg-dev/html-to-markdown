```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<pre><code class=\"language-python\">print('hello')</code></pre>", ConversionOptions())
}

```
