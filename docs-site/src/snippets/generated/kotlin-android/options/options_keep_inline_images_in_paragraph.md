---
id: fixture_kotlin_android_options_keep_inline_images_in_paragraph
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Text <img src='icon.png' alt='icon'> more text</p>", options)
}

```
