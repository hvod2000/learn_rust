use std::time::SystemTime;

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
        println!("");
    }
    println!("");
}

fn main() {
    print!("{}{}", "\x1b[?25l", "\n".repeat(5));
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => {
            let secs = duration.as_secs() % 60;
            let mins = duration.as_secs() / 60 % 60;
            let hours = duration.as_secs() / 3600 % 24;
            print(&format!("{:02}:{:02}:{:02}", (hours + 3) % 24, mins, secs));
        }
        Err(_) => panic!("PANIC!"),
    }
    print!("\n\x1b[?25h");
}
