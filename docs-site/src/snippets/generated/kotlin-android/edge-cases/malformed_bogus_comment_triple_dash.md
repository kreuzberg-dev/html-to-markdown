---
id: fixture_kotlin_android_malformed_bogus_comment_triple_dash
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<h1>One</h1>\n<!-- /// --->\n<p>Two</p>", ConversionOptions())
}

```
