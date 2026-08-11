---
id: fixture_kotlin_android_visitor_button_skip
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Actions available: <button>Save</button> <button>Delete</button> <button>Export</button></p>", ConversionOptions())
}

```
