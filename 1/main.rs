use std::io;

fn main() {
    let mut name = String::new();
    io::stdin().read_line(&mut name)
        .ok()
        .expect("Failed to read.");
    println!("Hello world! ~{}", name);
}
