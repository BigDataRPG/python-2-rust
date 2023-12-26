fn main() {

    let a = 1.23_f32; // `x` is a float 32 bits
    let b: f32 = 1.23_f32; // `x` is a float 32 bits

    // Not use varible, change from b to _b
    let _c = 1.23f32; // `x` is a float 32 bits
    let _d: f32 = 1.23; // `x` is a float 32 bits
    println!("Value of `a` is {}", a);
    println!("Value of `b` is {}", b);


    let f = 1.23_f64; // `f` is a float 64 bits
    let i = f as u8; // `i` is an unsigned integer 8 bits
    dbg!(i, f);

    let unsigned_integer: u32 = 42;
    let signed_integer: i32 = -42;

    println!("Unsigned Integer: {}", unsigned_integer);
    println!("Signed Integer: {}", signed_integer);


    let word: u16 = 128;
    let byte = word as i8;
    let ubyte = word as u8;
    dbg!(word, byte, ubyte);


    let too_big = 1000;
    let too_small = too_big as u8;
    dbg!(too_big, too_small);

    let left_val = 0.1 + 0.21;
    let right_val = 0.3;
    dbg!(left_val == right_val);


    let left_val: i64 = 1 + 2;
    let right_val: i64 = 3;
    dbg!(left_val == right_val);

        
    }