```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Part 1</p><hr><p>Part 2</p><hr><p>Part 3</p>", ConversionOptions())
}

```
