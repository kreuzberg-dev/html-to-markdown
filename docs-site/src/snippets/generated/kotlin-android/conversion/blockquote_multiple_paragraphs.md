---
id: fixture_kotlin_android_blockquote_multiple_paragraphs
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<blockquote><p>First paragraph.</p><p>Second paragraph.</p></blockquote>", ConversionOptions())
}

```
