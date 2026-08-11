---
id: fixture_kotlin_android_options_autolinks_false
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p><a href='https://example.com'>https://example.com</a></p>", options)
}

```
