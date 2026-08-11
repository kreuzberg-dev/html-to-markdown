---
id: fixture_kotlin_android_options_br_in_tables_false
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<table><tr><th>Col</th></tr><tr><td>A<br>B</td></tr></table>", options)
}

```
