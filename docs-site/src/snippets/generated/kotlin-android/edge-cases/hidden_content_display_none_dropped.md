---
id: fixture_kotlin_android_hidden_content_display_none_dropped
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>visible</p><div style=\"display:none\">secret hidden text</div><p>also visible</p>", ConversionOptions())
}

```
