---
id: fixture_kotlin_android_visitor_unknown_tag_preservation
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<article><p>Article text</p><x-custom>Custom element with content</x-custom><p>More article text</p></article>", ConversionOptions())
}

```
