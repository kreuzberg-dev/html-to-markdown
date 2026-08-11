---
id: fixture_kotlin_android_line_break_hr_tag
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Before rule.</p><hr><p>After rule.</p>", ConversionOptions())
}

```
