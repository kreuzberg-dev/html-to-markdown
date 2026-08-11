---
id: fixture_java_metadata_headers_hierarchy
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
        var optionsJson = "{\"extract_metadata\":true}";
var options = JsonUtil.fromJson(optionsJson, ConversionOptions.class);
        var result = io.xberg.htmltomarkdown.HtmlToMarkdownRs.convert("<html><head><title>Docs</title></head><body><h1>Introduction</h1><h2>Getting Started</h2><h3>Installation</h3><h3>Configuration</h3><h2>Advanced Usage</h2><h3>Custom Options</h3></body></html>", options);
    }
}

```
