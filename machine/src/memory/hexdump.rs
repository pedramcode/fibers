use std::{char, ops::Range};

use crate::memory::memory::Memory;

pub fn hexdump(mem: &Memory, range: Range<usize>) {
    let mut cur = range.start;
    let mut result = String::new();
    for chunk in mem.data[range].chunks(8) {
        let mut line = String::new();
        line.push_str(format!("0x{:08x} | ", cur).as_str());
        let mut printable = String::new();
        for byte in chunk {
            line.push_str(format!("{:08b} ", byte).as_str());
            if byte.is_ascii_graphic() {
                printable.push(char::from(*byte));
            } else {
                printable.push('.');
            }
        }
        line.push_str(" | ");
        line.push_str(printable.as_str());
        line.push_str("\n");
        result.push_str(line.as_str());
        cur += 8;
    }
    print!("{}", result);
}
