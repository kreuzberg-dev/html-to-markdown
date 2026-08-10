```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<img src=\"photo.jpg\" alt=\"A photo\">", ConversionOptions())
}

```
