---
id: fixture_kotlin_android_structured_data_json_ld
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<html><head><title>Article</title><script type=\"application/ld+json\">{\"@context\":\"https://schema.org\",\"@type\":\"Article\",\"headline\":\"My Article\",\"author\":{\"@type\":\"Person\",\"name\":\"Jane Doe\"},\"datePublished\":\"2024-01-15\"}</script></head><body><h1>My Article</h1><p>Article body text.</p></body></html>", ConversionOptions())
}

```
