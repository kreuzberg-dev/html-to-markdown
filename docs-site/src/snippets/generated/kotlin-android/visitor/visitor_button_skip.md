```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Actions available: <button>Save</button> <button>Delete</button> <button>Export</button></p>", ConversionOptions())
}

```
