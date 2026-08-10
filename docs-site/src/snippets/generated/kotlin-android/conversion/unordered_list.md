```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<ul><li>Item 1</li><li>Item 2</li><li>Item 3</li></ul>", ConversionOptions())
}

```
