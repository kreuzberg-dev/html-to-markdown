const std = @import("std");

pub fn main() !void {
    var threaded: std.Io.Threaded = .init(std.heap.smp_allocator, .{});
    defer threaded.deinit();

    var stdout_buffer: [64]u8 = undefined;
    var stdout_writer = std.Io.File.stdout().writer(threaded.io(), &stdout_buffer);
    const stdout = &stdout_writer.interface;

    try stdout.print("Example: module loaded successfully\n", .{});
    try stdout.flush();
}
