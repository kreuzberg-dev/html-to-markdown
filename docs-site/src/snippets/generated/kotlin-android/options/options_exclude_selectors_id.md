```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<body><div id=\"ad-container\">Buy stuff</div><p>Article text</p></body>", options)
}

```
