```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<div class='wrapper'><p>Inside div</p></div><p>Outside <span class='hl'>span text</span></p>", options)
}

```
