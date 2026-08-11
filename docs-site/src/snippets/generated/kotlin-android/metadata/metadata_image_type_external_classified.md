---
id: fixture_kotlin_android_metadata_image_type_external_classified
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p><img src=\"https://example.com/photo.jpg\" alt=\"A photo\"></p>", options)
}

```
