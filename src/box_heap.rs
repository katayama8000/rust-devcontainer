pub fn run() {
    println!(
        "{}{}{}box_heap.rs{}{}{}",
        "🦀", "🦀", "🦀", "🦀", "🦀", "🦀"
    );
    let byte_array = [b'h', b'e', b'l', b'l', b'o'];
    let byte_array = Box::new(byte_array);
    // print pointer address
    println!("byte_array: {:p}", byte_array);
    print_box(byte_array);
}

// error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
// fn print_u8(s: [u8]) {
//     println!("s: {:?}", s);
// }

// fn print_u16(s: [u16]) {
//     println!("s: {:?}", s);
// }

fn print_box(s: Box<[u8]>) {
    println!("s: {:?}", s);
}
