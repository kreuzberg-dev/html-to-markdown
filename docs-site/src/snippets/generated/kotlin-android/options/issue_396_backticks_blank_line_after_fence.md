---
id: fixture_kotlin_android_issue_396_backticks_blank_line_after_fence
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Foo</p><pre><code>1\n2\n</code></pre><p>Bar</p>", options)
}

```
