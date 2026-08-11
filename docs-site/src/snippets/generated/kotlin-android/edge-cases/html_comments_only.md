---
id: fixture_kotlin_android_html_comments_only
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<!-- This is a comment --><!-- Another comment -->", ConversionOptions())
}

```
