```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<a href=\"https://old-cdn.com/file.pdf\">Download</a>", ConversionOptions())
}

```
