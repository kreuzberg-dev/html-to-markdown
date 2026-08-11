---
id: fixture_kotlin_android_blockquote_text_then_paragraph_gets_blank_line
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<blockquote>Just text, then <p>a paragraph</p></blockquote>", ConversionOptions())
}

```
