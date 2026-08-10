```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<ul><li>Parent A<ul><li>Child A1</li><li>Child A2</li></ul></li><li>Parent B</li></ul>", ConversionOptions())
}

```
