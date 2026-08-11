---
id: fixture_kotlin_android_options_include_document_structure_false
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<article><h1>Heading</h1><p>Paragraph body.</p></article>", options)
}

```
