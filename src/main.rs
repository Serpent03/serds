/**
* @brief Data serialization and de-serialization using Rust.
* @todo Utilize XML data format to get the process done.
*/

use std::io::{ stdin, stdout, Write };

fn main() {
    let arr: [i32] = [1, 2, 3, 4, 5];
    // so this is interesting. apparently only one! owner at a time, which means that
    // you could trigger mutable stuff when the owner is some specific function scope, and NEVER
    // outside of that. Could make for a very VERY safe system that can support consistency.
    // also check pointers and dereferencing logic inside rust.
}
