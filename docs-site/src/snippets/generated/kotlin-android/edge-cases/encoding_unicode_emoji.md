```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Hello 🌍 World 🚀</p><p>Stars: ⭐ ✨</p>", ConversionOptions())
}

```
