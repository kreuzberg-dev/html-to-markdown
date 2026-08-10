```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<article><h1>Article</h1><p>Paragraph with <strong>bold</strong> and <em>italic</em>.</p><table><tr><th>Col</th></tr><tr><td>Val</td></tr></table><ul><li>Item 1</li><li>Item 2</li></ul></article>", ConversionOptions())
}

```
