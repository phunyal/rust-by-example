fn main() {
    // Variables can be type annotated.
    let logical: bool = true;
    println!("The value of logical is: {}", logical);

    let a_float: f64 = 1.0; // Regular annotation
    let an_integer = 5i32; // Suffix annotation
    println!("The value of a_float is: {}", a_float);
    println!("The value of an_integer is: {}", an_integer);

    // Or a default will be used.
    let default_float = 3.0; // f64
    let default_integer = 7; // i32
    println!("The value of default_float is: {}", default_float);
    println!("The value of default_integer is: {}", default_integer);

    // A type can also be inferred from context
    let mut inferred_type = 12;
    inferred_type = 4294967296i64;
    println!("The value of inferred_type is: {}", inferred_type);

    // A mutable variable's value can be changed.
    let mut mutable = 12;
    mutable = 21;
    println!("The value of mutable is: {}", mutable);

    // mutable = true; // ERROR: the type of a variable can't be changed

    // Variables can be overwritten with shadowing.
    let mutable = true;
    println!("The value of mutable is: {}", mutable);

    // Compound types - Array and Tuple

    // Array signature is fixed: [T; N] where T is the type of the elements and N is a compile-time constant for the number of elements in the array.
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of my_array is: {:?}", my_array);

    let my_tuple = (5u32, 1u8, true, -5.04f32);
    println!("The value of my_tuple is: {:?}", my_tuple);
}