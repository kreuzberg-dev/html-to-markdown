---
id: fixture_kotlin_android_visitor_element_start_skip_entire_subtree
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<div><h1>Title</h1><p>Content</p></div>", ConversionOptions())
}

```
