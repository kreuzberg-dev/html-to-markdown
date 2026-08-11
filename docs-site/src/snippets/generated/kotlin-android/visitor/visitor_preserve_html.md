---
id: fixture_kotlin_android_visitor_preserve_html
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<div><custom-tag>Custom content</custom-tag></div>", ConversionOptions())
}

```
