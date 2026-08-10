```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<html lang=\"es\"><head><title>Spanish Page</title></head><body><h1>Hola Mundo</h1><p>Este es un documento en español.</p></body></html>", options)
}

```
