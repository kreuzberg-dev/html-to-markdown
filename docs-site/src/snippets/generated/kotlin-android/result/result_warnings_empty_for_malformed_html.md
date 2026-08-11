---
id: fixture_kotlin_android_result_warnings_empty_for_malformed_html
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Unclosed paragraph<div>Mixed nesting</p></div>", ConversionOptions())
}

```
