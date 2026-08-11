---
id: fixture_kotlin_android_visitor_custom_emphasis
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>This is <em>important</em> text.</p>", ConversionOptions())
}

```
