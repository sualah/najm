



fn main() {
    let source_file = std::env::args().nth(1).expect("Usage: ./najm -- <source_file>");
    println!("Hello, world!, {}", source_file);
}
