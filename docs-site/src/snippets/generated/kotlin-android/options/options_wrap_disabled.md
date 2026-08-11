---
id: fixture_kotlin_android_options_wrap_disabled
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>This is a long paragraph that should not be wrapped at all because wrapping is disabled.</p>", options)
}

```
