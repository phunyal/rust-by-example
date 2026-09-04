#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    struct UnPrintable(i32);

    #[derive(Debug)]
    struct DebugPrintable(i32);

    println!("Now {:#?} will print!", DebugPrintable(3));

    println!("Now {:#?} will print!", Structure(1));
    println!("Now {:#?} will print!", Deep(Structure(2)));
    println!("Now {:#?} will print!", Person { name: "Alice", age: 30 });
}