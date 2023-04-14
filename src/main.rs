use std::io::{stdout, Write};

fn main() {
    let x = "y\n".repeat(4096).into_bytes();
    let mut s = stdout().lock();
    loop {
        s.write(&x).unwrap();
    }
}
