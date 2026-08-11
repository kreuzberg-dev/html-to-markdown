---
id: fixture_kotlin_android_metadata_link_type_email_classified
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Contact <a href=\"mailto:hello@example.com\">us</a> directly.</p>", options)
}

```
