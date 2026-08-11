---
id: fixture_kotlin_android_visitor_skip_images
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Before image</p><img src=\"photo.jpg\" alt=\"A photo\"><p>After image</p>", ConversionOptions())
}

```
