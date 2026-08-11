---
id: fixture_zig_metadata_microdata_schema_organization
language: zig
target: zig
level: typecheck
requires: []
side_effect: safe
---

```zig title="Zig"
const std = @import("std");
const html_to_markdown = @import("html_to_markdown");

pub fn main() !void {
    const _result_json = try html_to_markdown.convert("<html><head><title>Company</title></head><body><div itemscope itemtype=\"https://schema.org/Organization\"><span itemprop=\"name\">Acme Corp</span><span itemprop=\"foundingDate\">2020</span><span itemprop=\"url\">https://acmecorp.example.com</span><span itemprop=\"logo\">https://acmecorp.example.com/logo.png</span></div></body></html>", "{\"extract_metadata\":true}");
}

```
