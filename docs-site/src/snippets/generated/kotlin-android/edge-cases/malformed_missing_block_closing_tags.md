---
id: fixture_kotlin_android_malformed_missing_block_closing_tags
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<div><h1>Title<p>First paragraph<p>Second paragraph</div>", ConversionOptions())
}

```
