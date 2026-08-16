const std = @import("std");
const testing = std.testing;
const htm = @import("html_to_markdown_rs");

test "convert renders a basic heading to markdown" {
    const markdown = try htm.convert("<h1>Hello</h1>", null);
    defer std.heap.c_allocator.free(markdown);

    try testing.expect(std.mem.indexOf(u8, markdown, "# Hello") != null);
}

test "convert honours options passed as JSON" {
    const markdown = try htm.convert("<table><tr><td>a<br>b</td></tr></table>", "{\"br_in_tables\":true}");
    defer std.heap.c_allocator.free(markdown);

    try testing.expect(std.mem.indexOf(u8, markdown, "<br>") != null);
}

test "convert handles empty input without error" {
    const markdown = try htm.convert("", null);
    defer std.heap.c_allocator.free(markdown);

    try testing.expect(std.mem.indexOf(u8, markdown, "\"content\":\"\"") != null);
}
