```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>Background music:</p><audio src=\"music.ogg\" autoplay></audio><p>Enjoy!</p>", ConversionOptions.builder().build());
    }
}

```
