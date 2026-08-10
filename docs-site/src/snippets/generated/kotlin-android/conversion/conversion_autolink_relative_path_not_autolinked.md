```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<a href=\"/docs/intro.html\">/docs/intro.html</a>", ConversionOptions())
}

```
