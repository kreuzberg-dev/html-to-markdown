```java title="Java"
import io.xberg.htmltomarkdown.*;

public final class Example {
    public static void main(String[] args) throws Exception {
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<html><head><title>Fallback Title</title><meta property=\"og:title\" content=\"OG Title\"><meta property=\"og:description\" content=\"OG description text.\"><meta property=\"og:image\" content=\"https://example.com/image.jpg\"></head><body><p>Content</p></body></html>", ConversionOptions.builder().build());
    }
}

```
