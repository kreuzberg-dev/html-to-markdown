---
id: fixture_kotlin_android_link_with_bold_text
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<a href=\"https://example.com\"><strong>Bold link</strong></a>", ConversionOptions())
}

```
