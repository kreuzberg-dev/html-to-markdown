---
id: fixture_kotlin_android_visitor_link_bare_string_preserves_case
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<a href=\"https://old-cdn.com/file.pdf\">Download</a>", ConversionOptions())
}

```
