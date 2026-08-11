---
id: fixture_kotlin_android_blockquote_nested_list_indentation_preserved
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<blockquote><ul><li>item a<ul><li>sub a1</li></ul></li></ul></blockquote>", ConversionOptions())
}

```
