/**
* @brief Data serialization and de-serialization using Rust.
* @todo Utilize XML data format to get the process done.
*/

#[allow(unused)]
use std::io::{ stdin, stdout, Write };


fn print_to_byte(arr: &[i32], n: u32) -> () {
    for i in 0..n {
        print!("{}", arr[i as usize]);
    }
    println!();
}

fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    print_to_byte(&arr, 5);

    // so new news. it looks like & allows you to "borrow" that reference for use in another scope.
}
