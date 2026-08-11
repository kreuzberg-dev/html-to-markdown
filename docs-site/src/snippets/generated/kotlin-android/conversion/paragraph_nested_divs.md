---
id: fixture_kotlin_android_paragraph_nested_divs
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<div><div><p>Nested text</p></div></div>", ConversionOptions())
}

```
