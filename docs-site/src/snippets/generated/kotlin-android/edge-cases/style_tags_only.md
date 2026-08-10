```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<html><head><style>body { color: red; }</style></head><body><style>.foo { margin: 0; }</style></body></html>", ConversionOptions())
}

```
