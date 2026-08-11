---
id: fixture_kotlin_android_visitor_skip_heading
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<h1>Title</h1><p>Body text remains.</p>", ConversionOptions())
}

```
