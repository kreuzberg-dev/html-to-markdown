```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<a href=\"foobar.png\">foobar.png</a> <a href=\"https://www.heise.de\">https://www.heise.de</a>", ConversionOptions())
}

```
