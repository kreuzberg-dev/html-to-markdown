---
id: fixture_kotlin_android_options_max_depth_truncates
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<div><p>Shallow</p><div><div><div><p>Too deep</p></div></div></div></div>", options)
}

```
