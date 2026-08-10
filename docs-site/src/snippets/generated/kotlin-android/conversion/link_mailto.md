```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<a href=\"mailto:user@example.com\">Email us</a>", ConversionOptions())
}

```
