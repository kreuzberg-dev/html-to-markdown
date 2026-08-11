---
id: fixture_kotlin_android_image_linked
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<a href=\"https://example.com\"><img src=\"icon.png\" alt=\"Icon\"></a>", ConversionOptions())
}

```
