---
id: fixture_kotlin_android_options_link_style_reference
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p><a href='https://example.com'>Example</a> and <a href='https://other.com'>Other</a></p>", options)
}

```
