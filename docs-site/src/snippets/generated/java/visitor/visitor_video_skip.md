---
id: fixture_java_visitor_video_skip
language: java
target: java
level: typecheck
requires: []
side_effect: safe
---

```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<h2>Demo</h2><video src=\"demo.webm\"></video><p>See the demo above.</p>", ConversionOptions.builder().build());
    }
}

```
