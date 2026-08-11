---
id: fixture_kotlin_android_visitor_underline_custom
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>This is <u>very important</u> text.</p>", ConversionOptions())
}

```
