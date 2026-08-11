---
id: fixture_kotlin_android_emphasis_strikethrough_del
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p><del>deleted text</del></p>", ConversionOptions())
}

```
