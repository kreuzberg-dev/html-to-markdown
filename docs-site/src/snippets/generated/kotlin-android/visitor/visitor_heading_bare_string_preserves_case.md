---
id: fixture_kotlin_android_visitor_heading_bare_string_preserves_case
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<h2>Important Section Title</h2><p>Body.</p>", ConversionOptions())
}

```
