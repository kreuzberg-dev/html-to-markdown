```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<body><div class=\"nav\">Navigation</div><p>Article body</p></body>", options)
}

```
