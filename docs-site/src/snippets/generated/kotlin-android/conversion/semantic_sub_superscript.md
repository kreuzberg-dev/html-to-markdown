---
id: fixture_kotlin_android_semantic_sub_superscript
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>H<sub>2</sub>O and E=mc<sup>2</sup></p>", ConversionOptions())
}

```
