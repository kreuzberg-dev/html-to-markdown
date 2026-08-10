```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<html><head><title>Docs</title></head><body><h1>Introduction</h1><h2>Getting Started</h2><h3>Installation</h3><h3>Configuration</h3><h2>Advanced Usage</h2><h3>Custom Options</h3></body></html>", options)
}

```
