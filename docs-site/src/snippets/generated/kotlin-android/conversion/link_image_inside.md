---
id: fixture_kotlin_android_link_image_inside
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<a href=\"https://example.com\"><img src=\"logo.png\" alt=\"Logo\"></a>", ConversionOptions())
}

```
