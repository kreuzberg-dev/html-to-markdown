---
id: fixture_kotlin_android_visitor_skip_strong
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Normal <strong>bold text</strong> normal</p>", ConversionOptions())
}

```
