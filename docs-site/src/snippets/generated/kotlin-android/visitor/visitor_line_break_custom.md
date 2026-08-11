---
id: fixture_kotlin_android_visitor_line_break_custom
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>First line<br>Second line<br>Third line</p>", ConversionOptions())
}

```
