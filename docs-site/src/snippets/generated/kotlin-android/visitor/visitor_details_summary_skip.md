```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Main content here.</p><details><summary>Hidden section</summary><p>Secret details</p></details><p>More main content.</p>", ConversionOptions())
}

```
