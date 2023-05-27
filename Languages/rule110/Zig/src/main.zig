const std = @import("std");
const WIDTH: usize = 100;

// fn isAlive(a: bool, b: bool, c: bool) bool {
//     const Bits = packed struct { bool, bool, bool };
//     switch (@bitCast(u3, Bits{ true, true, true })) {
//         @bitCast(u3, Bits{ true, false, true }) => return true,
//         @bitCast(u3, Bits{ false, false, false }) => return false,
//         else => unrechable,
//     }
// }
fn isAlive(a: bool, b: bool, c: bool) bool {
    if (a and b and c) {
        return false;
    } else if (a and b and !c) {
        return true;
    } else if (a and !b and c) {
        return true;
    } else if (a and !b and !c) {
        return false;
    } else if (!a and b and !c) {
        return true;
    } else if (!a and !b and c) {
        return true;
    } else if (!a and !b and !c) {
        return false;
    }
    return false;
}

fn display(array: []bool) void {
    for (array) |elem| {
        var item: u8 = undefined;
        if (elem) {
            item = '#';
        } else {
            item = ' ';
        }
        std.debug.print("{c}", .{item});
    }
    std.debug.print("\n", .{});
}

fn range(len: usize) []const void {
    return @as([*]void, undefined)[0..len];
}

fn next_gen(last_gen: []bool) [WIDTH]bool {
    var next: [WIDTH]bool = undefined;
    for (range(WIDTH)) |_, i| {
        const a: bool = last_gen[((i -% 1) +% WIDTH) % WIDTH];
        const b: bool = last_gen[i];
        const c: bool = last_gen[((i + 1) + WIDTH) % WIDTH];
        next[i] = isAlive(a, b, c);
    }
    return next;
}

pub fn main() void {
    var generation: [WIDTH]bool = undefined;
    generation[WIDTH - 1] = true;
    while (true) {
        display(&generation);
        generation = next_gen(&generation);
        std.time.sleep(100000000);
    }
    display(&generation);
}
