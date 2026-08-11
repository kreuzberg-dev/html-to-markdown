---
id: fixture_java_encoding_cjk_characters
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
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<p>中文内容</p><p>日本語テキスト</p><p>한국어 텍스트</p>", ConversionOptions.builder().build());
    }
}

```
