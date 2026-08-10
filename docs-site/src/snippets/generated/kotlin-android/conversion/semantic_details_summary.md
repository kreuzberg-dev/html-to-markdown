```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<details><summary>Click to expand</summary><p>Hidden content here.</p></details>", ConversionOptions())
}

```
