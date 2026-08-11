---
id: fixture_kotlin_android_semantic_article
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<article><h2>Article Title</h2><p>Article body.</p></article>", ConversionOptions())
}

```
