---
id: fixture_kotlin_android_code_with_backticks_in_content
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Use <code>`backtick` here</code> carefully.</p>", ConversionOptions())
}

```
