---
id: fixture_kotlin_android_hidden_content_visibility_hidden_dropped
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>visible</p><span style=\"visibility:hidden\">secret hidden span</span><p>also visible</p>", ConversionOptions())
}

```
