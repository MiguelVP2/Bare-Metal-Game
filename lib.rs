#![no_std]

use pluggable_interrupt_os::vga_buffer::{BUFFER_HEIGHT, clear_row, Color};

pub fn clear_screen() {
    for row in 0..BUFFER_HEIGHT {
        clear_row(row, Color::Black);
    }
}