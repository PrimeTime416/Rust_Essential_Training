// use core::num::dec2flt::number;

use rand::prelude::*;


fn main() {
    let number = random::<f64>();
    println!("Randum number is: {:?}", number);

    let number = thread_rng().gen_range(1..11);
    println!("Randum number is: {:?}", number);
    
}
