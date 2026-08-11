---
id: fixture_kotlin_android_visitor_horizontal_rule_skip
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Part 1</p><hr><p>Part 2</p><hr><p>Part 3</p>", ConversionOptions())
}

```
