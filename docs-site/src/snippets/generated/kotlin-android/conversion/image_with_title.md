---
id: fixture_kotlin_android_image_with_title
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<img src=\"chart.png\" alt=\"Sales chart\" title=\"Q3 Sales\">", ConversionOptions())
}

```
