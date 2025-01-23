const std = @import("std");

pub fn main() !void {
    // Prints to stderr (it's a shortcut based on `std.io.getStdErr()`)
    // TODO print pic of waifu

    while (true) {
        // stdout is for the actual output of your application, for example if you
        // are implementing gzip, then only the compressed bytes should be sent to
        // stdout, not any debugging messages.
        const stdout_file = std.io.getStdOut().writer();
        var bw = std.io.bufferedWriter(stdout_file);
        const stdout = bw.writer();
        _ = try stdout.write("$: ");

        try bw.flush(); // don't forget to flush!

        // get stdin from user
        const stdin_file = std.io.getStdIn().reader();
        var br = std.io.bufferedReader(stdin_file);
        const stdin = br.reader();
        var msg_buf: [4096]u8 = undefined;
        const msg = try stdin.readUntilDelimiterOrEof(&msg_buf, '\n');

        if (msg) |m| {
            _ = try stdout.write(m);
            _ = try stdout.write("\n");
            try bw.flush(); // don't forget to flush!
        } else {
            break;
        }
    }
}

test "simple test" {
    var list = std.ArrayList(i32).init(std.testing.allocator);
    defer list.deinit(); // try commenting this out and see if zig detects the memory leak!
    try list.append(42);
    try std.testing.expectEqual(@as(i32, 42), list.pop());
}
