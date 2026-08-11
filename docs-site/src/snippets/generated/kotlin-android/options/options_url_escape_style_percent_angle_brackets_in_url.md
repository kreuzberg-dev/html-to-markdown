---
id: fixture_kotlin_android_options_url_escape_style_percent_angle_brackets_in_url
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<a href=\"/file (1) <draft>.pdf\">file</a>", options)
}

```
