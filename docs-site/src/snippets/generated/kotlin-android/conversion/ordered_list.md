```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<ol><li>First</li><li>Second</li><li>Third</li></ol>", ConversionOptions())
}

```
