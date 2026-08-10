```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>Listen to this: <audio src=\"podcast.mp3\" controls></audio></p>", ConversionOptions.builder().build());
    }
}

```
