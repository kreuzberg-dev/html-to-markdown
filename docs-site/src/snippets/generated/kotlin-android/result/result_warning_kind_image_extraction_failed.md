---
id: fixture_kotlin_android_result_warning_kind_image_extraction_failed
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Text<img src=\"data:BADMIME\" alt=\"broken\">end</p>", options)
}

```
