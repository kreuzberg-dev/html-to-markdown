---
id: fixture_kotlin_android_options_preserve_tags_iframe
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<p>Before</p><iframe src='video.html' width='560'></iframe><p>After</p>", options)
}

```
