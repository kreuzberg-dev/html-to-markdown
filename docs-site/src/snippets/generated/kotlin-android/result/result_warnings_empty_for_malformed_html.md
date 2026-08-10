```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Unclosed paragraph<div>Mixed nesting</p></div>", ConversionOptions())
}

```
