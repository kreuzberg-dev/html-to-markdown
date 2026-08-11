---
id: fixture_kotlin_android_visitor_mark_skip
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Key insight: <mark>always validate input</mark> for security.</p>", ConversionOptions())
}

```
