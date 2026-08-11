---
id: fixture_zig_visitor_preserve_html
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
    var gpa: std.heap.DebugAllocator(.{}) = .init;
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const c = html_to_markdown.c;
    const TestVisitor_visitor_preserve_html = struct {
        pub fn visit_custom_element(_ctx: [*c]const c.HTMHtmContext, _user_data: ?*anyopaque, _tag_name: [*c]const u8, _html: [*c]const u8, out_custom: [*c][*c]u8, out_len: [*c]usize) callconv(.c) i32 {
        _ = _ctx;
        _ = _user_data;
        _ = _tag_name;
        _ = _html;
        _ = out_custom;
        _ = out_len;
        return 3;
        }
    };
    var _callbacks: c.HTMHtmVisitorCallbacks = std.mem.zeroes(c.HTMHtmVisitorCallbacks);
    _callbacks.visit_custom_element = &TestVisitor_visitor_preserve_html.visit_custom_element;
    const _visitor = html_to_markdown.c.htm_visitor_create(&_callbacks);
    defer html_to_markdown.c.htm_visitor_free(_visitor);
    const _options_z = try std.heap.c_allocator.dupeZ(u8, "{}");
    defer std.heap.c_allocator.free(_options_z);
    const _options = html_to_markdown.c.htm_conversion_options_from_json(_options_z.ptr);
    defer html_to_markdown.c.htm_conversion_options_free(_options);
    html_to_markdown.c.htm_options_set_visitor(_options, _visitor);
    const _html_z = try std.heap.c_allocator.dupeZ(u8, "<div><custom-tag>Custom content</custom-tag></div>");
    defer std.heap.c_allocator.free(_html_z);
    const _result = html_to_markdown.c.htm_convert(_html_z.ptr, _options);
    try testing.expect(_result != null);
    defer html_to_markdown.c.htm_conversion_result_free(_result.?);
    const _json_ptr = html_to_markdown.c.htm_conversion_result_to_json(_result.?);
    defer html_to_markdown.c.htm_free_string(_json_ptr);
    const _result_json = std.mem.sliceTo(_json_ptr, 0);
    var _parsed = try std.json.parseFromSlice(std.json.Value, allocator, _result_json, .{});
    defer _parsed.deinit();
    const result = &_parsed.value;
    }
}

```
