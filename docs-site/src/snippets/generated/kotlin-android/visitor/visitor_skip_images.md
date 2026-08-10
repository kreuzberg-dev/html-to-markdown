```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Before image</p><img src=\"photo.jpg\" alt=\"A photo\"><p>After image</p>", ConversionOptions())
}

```
