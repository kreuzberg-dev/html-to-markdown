---
id: fixture_kotlin_android_visitor_image_bare_string_preserves_case
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<img src=\"PhotoOne.JPG\" alt=\"Sunset Over Bay\">", ConversionOptions())
}

```
