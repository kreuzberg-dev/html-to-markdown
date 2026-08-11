---
id: fixture_kotlin_android_semantic_details_summary
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<details><summary>Click to expand</summary><p>Hidden content here.</p></details>", ConversionOptions())
}

```
