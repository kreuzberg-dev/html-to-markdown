---
id: fixture_kotlin_android_metadata_link_type_external_classified
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>See <a href=\"https://example.com\">Example</a> for details.</p>", options)
}

```
