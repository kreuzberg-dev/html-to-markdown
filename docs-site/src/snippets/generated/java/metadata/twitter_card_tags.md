---
id: fixture_java_twitter_card_tags
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<html><head><meta name=\"twitter:card\" content=\"summary_large_image\"><meta name=\"twitter:site\" content=\"@examplesite\"><meta name=\"twitter:title\" content=\"Twitter Card Title\"><meta name=\"twitter:description\" content=\"Twitter card description.\"><meta name=\"twitter:image\" content=\"https://example.com/twitter-image.jpg\"></head><body><p>Content</p></body></html>", ConversionOptions.builder().build());
    }
}

```
