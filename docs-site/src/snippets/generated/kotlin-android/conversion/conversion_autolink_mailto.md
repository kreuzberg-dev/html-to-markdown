---
id: fixture_kotlin_android_conversion_autolink_mailto
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<a href=\"mailto:a@b.com\">a@b.com</a>", ConversionOptions())
}

```
