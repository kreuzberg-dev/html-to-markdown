```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Visit <a href=\"https://example.com\">Example</a> for more info.</p>", ConversionOptions())
}

```
