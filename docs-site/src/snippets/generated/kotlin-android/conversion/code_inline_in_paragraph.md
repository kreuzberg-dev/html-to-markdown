---
id: fixture_kotlin_android_code_inline_in_paragraph
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Call the <code>initialize()</code> method first.</p>", ConversionOptions())
}

```
