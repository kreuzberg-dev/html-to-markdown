---
id: fixture_kotlin_android_options_list_indent_tabs
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<ul><li>Parent<ul><li>Child</li></ul></li></ul>", options)
}

```
