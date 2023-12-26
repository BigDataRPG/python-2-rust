use core::num;

fn main() {

        // Integers
        let a = 1; // `a` is an immutable variable
        let mut b = 2; // `b` is a mutable variable

        println!("First value of `a` is {}", a);
        println!("First value of `b` is {}", b);

        // a += 1; // Increment `a` by 1
        b += 1; // Increment `b` by 1

        println!("Second value of a = {}, Second value of b = {}", a, b);

        // Floats
        // Type in rust can be declared before and after the value name
        let c = 1.0; // `c` is a float 64 bits
        let d = 3.0f32; // `e` is a float 32 bits, decalre type after value name
        let f: f32 = -1.732; // `f` is a single precision float

        println!("Value of `c` is {}", c);
        println!("Value of `d` is {}", d);
        println!("Value of `f` is {}", f);

        // Complex numbers
        // Complex numbers are not part of the core library
        // let complex_integer = num::complex::Complex::new(1, 2);
        // let complex_float = num::complex::Complex::new(1.0, 2.0);
        // println!("Value of `complex_integer` is {}", complex_integer);
        // println!("Value of `complex_float` is {}", complex_float);

        // Booleans
        let yes = true;
        let no: bool = false; // with explicit type annotation
        println!("Value of `yes` is {}", yes);
        println!("Value of `no` is {}", no);


        // Binary literals
        let bin_x = 0b1010; // `bin_x` is declared as binary 1010
        let bin_y = 0b1111_0000; // `bin_y` is declared as binary 1111_0000
        dbg!(bin_x);
        dbg!(bin_y);
        dbg!(bin_y | bin_x);


        // Hex literals
        let hex_x = 0x1234; // `hex_x` is declared as hexadecimal 1234
        let hex_y = 0xff; // `hex_y` is declared as hexadecimal ff
        println!("Value of `hex_x` is {}", hex_x);
        println!("Value of `hex_y` is {}", hex_y);


    }