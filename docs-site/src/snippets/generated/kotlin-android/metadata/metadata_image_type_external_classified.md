```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p><img src=\"https://example.com/photo.jpg\" alt=\"A photo\"></p>", options)
}

```
