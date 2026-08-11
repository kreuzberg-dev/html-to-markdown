---
id: fixture_kotlin_android_options_url_escape_style_percent_image
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<img src=\"/img (1) <draft>.png\" alt=\"alt\">", options)
}

```
