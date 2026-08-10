```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>The <abbr title=\"World Wide Web\">WWW</abbr> is global.</p>", ConversionOptions())
}

```
