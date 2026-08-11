---
id: fixture_zig_options_preprocessing_remove_navigation_false_keeps_nav
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
    const _result_json = try html_to_markdown.convert("<nav>SiteMenu</nav><main><p>MainContent</p></main><aside>SidebarText</aside>", "{\"preprocessing\":{\"remove_navigation\":false}}");
}

```
