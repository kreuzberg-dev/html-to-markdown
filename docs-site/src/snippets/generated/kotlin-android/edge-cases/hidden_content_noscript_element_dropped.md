---
id: fixture_kotlin_android_hidden_content_noscript_element_dropped
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>visible</p><noscript><p>secret noscript text</p></noscript><p>also visible</p>", ConversionOptions())
}

```
