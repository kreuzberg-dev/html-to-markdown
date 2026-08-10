```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Einstein's E=mc<sup>2</sup> revolutionized physics.</p>", ConversionOptions())
}

```
