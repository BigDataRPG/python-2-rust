fn main() {

    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array: {:?}", xs);
    println!("first element: {}", xs[0]);
    println!("second element: {}", xs[1]);
    println!("element 2 - 4: {:?}", &xs[1..4]);
    println!("size of the array: {}", xs.len());

    let ys: [i32; 500] = [0; 500];
    println!("array: {:?}", &ys[2..10]);
    println!("size of the array: {}", ys.len());

}