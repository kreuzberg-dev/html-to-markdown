---
id: fixture_kotlin_android_options_code_block_backticks
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<pre><code class=\"language-js\">console.log('hi');</code></pre>", options)
}

```
