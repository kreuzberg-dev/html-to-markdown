```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<html lang=\"en\" dir=\"ltr\"><head><title>LTR Document</title></head><body><p>This is left-to-right text.</p></body></html>", options)
}

```
