---
id: fixture_kotlin_android_visitor_underline_skip
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Normal text with <u>underlined part</u> and more text.</p>", ConversionOptions())
}

```
