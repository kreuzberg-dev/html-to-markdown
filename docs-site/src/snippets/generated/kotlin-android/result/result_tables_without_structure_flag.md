---
id: fixture_kotlin_android_result_tables_without_structure_flag
language: kotlin
target: kotlin_android
level: typecheck
requires: []
side_effect: safe
---

```kotlin title="Kotlin (Android)"
import io.xberg.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = IoXbergAndroidHtmlToMarkdownRs.convert("<table><tr><th>X</th></tr><tr><td>Y</td></tr></table>", ConversionOptions())
}

```
