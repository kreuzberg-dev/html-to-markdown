---
id: fixture_kotlin_android_options_preprocessing_remove_forms
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Before</p><form><input type='text'/><button>Submit</button></form><p>After</p>", options)
}

```
