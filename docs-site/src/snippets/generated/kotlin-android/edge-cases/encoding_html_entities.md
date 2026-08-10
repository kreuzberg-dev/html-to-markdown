```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>&amp; &lt; &gt; &nbsp; &quot; &apos;</p>", ConversionOptions())
}

```
