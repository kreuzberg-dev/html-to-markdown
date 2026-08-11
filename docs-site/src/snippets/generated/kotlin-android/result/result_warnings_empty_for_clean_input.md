---
id: fixture_kotlin_android_result_warnings_empty_for_clean_input
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<h1>Title</h1><p>Clean content with <a href='https://example.com'>a link</a>.</p>", ConversionOptions())
}

```
