---
id: fixture_kotlin_android_unordered_list
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<ul><li>Item 1</li><li>Item 2</li><li>Item 3</li></ul>", ConversionOptions())
}

```
