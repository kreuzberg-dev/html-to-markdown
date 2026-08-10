```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Contact <a href=\"mailto:hello@example.com\">us</a> directly.</p>", options)
}

```
