use std::char;
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

    let f:usize = 200;
    let f_size = mem::size_of_val(&f);
    println!("f = {} and it's size {} your computer's architecture {} ", f, f_size, f_size*8);

    let g:char = 'g'; // _,;+%'!? can be char
    println!("g = {} and it's size {}", g,mem::size_of_val(&g));

    // f32 or f64 can't be u , it is signed by default IEEE754 ile none, can be assigned +- infinite values
    let h:f32 = 2.00000000005;
    println!("h = {} and it's size {}", h,mem::size_of_val(&h));

    let i:bool = false; // true
    println!("i = {} and it's size {}", i,mem::size_of_val(&i));
}
