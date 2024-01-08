#[allow(dead_code)]
#[allow(unused_variables)]

use std::mem;

fn main() {
    // println!("Hello, world!");
    let a : u8 = 125; // u = unsigned 0-255 8 bit
    println!("a = {}", a);

    // u unsigned, 0 - 2^N-1
    // i signed. -2^(N-1) ------- 2^(N-1)-1
    let b : i8 = 0; // -128 --- 127

    print!("b = {}", b);
    // b = 44;

    let mut c : i8 = 0;
    print!("c = {}", c);
    c = 12;
    print!("c = {}", c);

    // let d : = 123456789;
    let d = 123456789;
    print!("d = {} and size {} byte", d,mem::size_of_val(&d));

    let e:isize = -200;
    let e_size = mem::size_of_val(&e);
    println!("e = {} and it's size {} your computer's architecture {} ", e, e_size, e_size*8);
}
