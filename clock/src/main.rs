use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread, time};
use chrono::Local;
use ctrlc;

const FONT3X5: [u16; 12] = [
    0x0000, // space
    0x7e3f, 0x07f1, 0x76b7, 0x7eb5, 0x7c9c, 0x5ebd, 0x5ebf, 0x62f0, 0x7ebf, 0x7ebd, // 0-9
    0x0140, // colon
];

fn print(message: &str) {
    println!("{}", "\x1b[A".repeat(5));
    for row in (0..=2).rev() {
        print!(" ");
        for char in message.chars() {
            print!(" ");
            let symbol = match char {
                '0'..=':' => FONT3X5[char as usize - 47],
                _ => FONT3X5[0],
            };
            for x in 0..=2 {
                let pixel = symbol >> (5 * x + row * 2) & 3;
                let repr = match if row != 2 { pixel } else { pixel & 1 } {
                    0 => " ",
                    1 => "▄",
                    2 => "▀",
                    _ => "█",
                };
                print!("{}", repr);
            }
        }
        println!();
    }
    println!();
}

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    print!("\x1b[?25l{}", "\n".repeat(5));
    while running.load(Ordering::SeqCst) {
        print(&format!("{}", Local::now().format("%H:%M:%S")));
        thread::sleep(time::Duration::from_millis(500));
    }
    println!("\x1b[?25h");
}
