
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;

    (boolean, integer)
}

// Must be declared before use struct
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    let many_types = (1i32, 2.3f64, "hello", true);

    println!("first value: {}", many_types.0);
    println!("second value: {}", many_types.1);

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair: (i32, bool) = (1, true);
    println!("pair is {:?}", pair);
    println!("the reversed pair is {:?}", reverse(pair));

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);

    let tuple = (1, 2, 3, 4, 5);
    let (a, b, c, d, e) = tuple;
    println!("{}, {}, {}, {}, {}", a, b, c, d, e);
}



