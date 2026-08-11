---
id: fixture_kotlin_android_visitor_skip_code_blocks
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Intro text</p><pre><code>let x = 42;</code></pre><p>Outro text</p>", ConversionOptions())
}

```
