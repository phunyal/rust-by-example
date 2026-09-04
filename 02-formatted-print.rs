fn main() {
    println!("{} days", 31);

    // Positional arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // Named arguments can be used.
    println!("{subject} {verb} {object}",
                object="the lazy dog",
                subject="the quick brown fox",
                verb="jumps over");

    println!("Base 10: {}", 69420);
    println!("Base 2: {:b}", 69420);
    println!("Base 16: {:x}", 69420);
    println!("Base 8: {:o}", 69420);

    // Right-justified with specified width.
    println!("{number:>5}", number = 1);

    // Left-justified with specified width.
    println!("{number:<5}", number = 1);

    println!("{number:0>width$}", number = 1, width = 5);

    // FIXME
    println!("My name is {0}, {1} {0}", "Bond", "James");

    #[allow(dead_code)]
    struct Structure(i32);

    let number: f64 = 1.0;
    let width: usize = 6;

    println!("{number:0>width$}");

    let pi = 3.141592;

    println!("{pi:.3}");
}