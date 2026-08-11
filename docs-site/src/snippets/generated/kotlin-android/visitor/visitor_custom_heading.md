---
id: fixture_kotlin_android_visitor_custom_heading
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<h2>Section Title</h2><p>Content below heading.</p>", ConversionOptions())
}

```
