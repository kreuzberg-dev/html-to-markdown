```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<details><summary>Click to expand</summary><p>This content is initially hidden.</p><p>But can be revealed by the user.</p></details>", ConversionOptions.builder().build());
    }
}

```
