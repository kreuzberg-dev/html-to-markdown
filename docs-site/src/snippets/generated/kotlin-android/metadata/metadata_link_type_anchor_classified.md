---
id: fixture_kotlin_android_metadata_link_type_anchor_classified
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Jump to <a href=\"#section\">section</a> below.</p>", options)
}

```
