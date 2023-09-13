fn main() {
    println!(
        "Hello, {}!",
        std::env::args()
            .nth(1)
            .unwrap_or_else(|| "world".to_string())
    );
}
