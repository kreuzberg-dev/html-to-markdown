---
id: fixture_kotlin_android_encoding_html_entities
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>&amp; &lt; &gt; &nbsp; &quot; &apos;</p>", ConversionOptions())
}

```
