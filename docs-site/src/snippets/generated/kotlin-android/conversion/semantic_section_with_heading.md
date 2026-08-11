---
id: fixture_kotlin_android_semantic_section_with_heading
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<section><h3>Section Heading</h3><p>Section content.</p></section>", ConversionOptions())
}

```
