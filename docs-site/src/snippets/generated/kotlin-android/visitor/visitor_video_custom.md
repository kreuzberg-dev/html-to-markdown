```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Watch our tutorial:</p><video src=\"tutorial.mp4\" width=\"320\" height=\"240\" controls></video><p>Great content!</p>", ConversionOptions())
}

```
