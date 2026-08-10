```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<img src=\"PhotoOne.JPG\" alt=\"Sunset Over Bay\">", ConversionOptions())
}

```
