```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<div><div><div><div><p>Deep content</p></div></div></div></div>", ConversionOptions())
}

```
