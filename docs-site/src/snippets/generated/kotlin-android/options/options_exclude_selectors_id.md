---
id: fixture_kotlin_android_options_exclude_selectors_id
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<body><div id=\"ad-container\">Buy stuff</div><p>Article text</p></body>", options)
}

```
