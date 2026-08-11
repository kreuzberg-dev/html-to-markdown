---
id: fixture_kotlin_android_emphasis_strikethrough_s
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p><s>strikethrough</s></p>", ConversionOptions())
}

```
