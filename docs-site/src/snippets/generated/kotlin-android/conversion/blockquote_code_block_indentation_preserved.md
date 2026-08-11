---
id: fixture_kotlin_android_blockquote_code_block_indentation_preserved
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<blockquote><pre><code>line1\n    line2 indented</code></pre></blockquote>", ConversionOptions())
}

```
