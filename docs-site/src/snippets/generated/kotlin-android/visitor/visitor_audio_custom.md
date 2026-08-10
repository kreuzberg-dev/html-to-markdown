```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Listen to this: <audio src=\"podcast.mp3\" controls></audio></p>", ConversionOptions())
}

```
