```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<article><h2>Article Title</h2><p>Article body.</p></article>", ConversionOptions())
}

```
