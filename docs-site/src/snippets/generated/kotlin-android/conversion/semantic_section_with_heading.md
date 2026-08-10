```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<section><h3>Section Heading</h3><p>Section content.</p></section>", ConversionOptions())
}

```
