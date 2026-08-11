---
id: fixture_kotlin_android_paragraph_multiple
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>First paragraph.</p><p>Second paragraph.</p>", ConversionOptions())
}

```
