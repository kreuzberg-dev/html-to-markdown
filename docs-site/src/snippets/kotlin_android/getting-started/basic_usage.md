```kotlin
import io.xberg.android.HtmlToMarkdown

val html = "<h1>Hello</h1><p>This is <strong>fast</strong>!</p>"
val result = HtmlToMarkdown.convert(html)
val markdown: String? = result.content
```
