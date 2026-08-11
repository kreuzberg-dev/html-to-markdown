---
id: fixture_kotlin_android_paragraph_with_line_breaks
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Line one.<br>Line two.<br>Line three.</p>", ConversionOptions())
}

```
