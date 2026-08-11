---
id: fixture_kotlin_android_visitor_custom_link_format
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Visit <a href=\"https://example.com\">Example</a> for more info.</p>", ConversionOptions())
}

```
