```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Background music:</p><audio src=\"music.ogg\" autoplay></audio><p>Enjoy!</p>", ConversionOptions())
}

```
