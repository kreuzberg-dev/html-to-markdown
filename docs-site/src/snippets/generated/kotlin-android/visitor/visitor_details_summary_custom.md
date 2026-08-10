```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<details><summary>Click to expand</summary><p>This content is initially hidden.</p><p>But can be revealed by the user.</p></details>", ConversionOptions())
}

```
