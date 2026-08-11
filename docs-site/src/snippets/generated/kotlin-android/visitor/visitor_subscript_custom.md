---
id: fixture_kotlin_android_visitor_subscript_custom
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>H<sub>2</sub>O is water.</p>", ConversionOptions())
}

```
