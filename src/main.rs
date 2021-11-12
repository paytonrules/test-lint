#![deny(missing_debug_implementations)]

mod thingy;
use thingy::Whatever;

fn main() {
    let _a = Whatever {};
    println!("Hello, world!");
}
