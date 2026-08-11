---
id: fixture_kotlin_android_ordered_list
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<ol><li>First</li><li>Second</li><li>Third</li></ol>", ConversionOptions())
}

```
